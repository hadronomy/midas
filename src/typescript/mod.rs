use std::rc::Rc;
use std::sync::Arc;

use deno_core::{ModuleSpecifier, *};
use deno_runtime::BootstrapOptions;
use deno_runtime::deno_fs::RealFs;
use deno_runtime::deno_permissions::PermissionsContainer;
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use deno_runtime::worker::{MainWorker, WorkerOptions, WorkerServiceOptions};
use miette::*;
use module_loader::TypescriptModuleLoader;
use serde::Deserialize;

mod module_loader;

static WORKER_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/CLI_SNAPSHOT.bin"));

pub struct TsRunner {
    permissions: PermissionsContainer,
    main_module: ModuleSpecifier,
    fs: Arc<RealFs>,
    worker: Option<MainWorker>,
}

impl TsRunner {
    pub fn new(main_module: ModuleSpecifier) -> Self {
        let fs = Arc::new(RealFs);
        let permission_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
        let permissions = PermissionsContainer::allow_all(permission_parser);

        let mut runner = Self { permissions, main_module, fs, worker: None };

        runner.worker = Some(runner.create_worker());
        runner
    }

    fn create_worker(&self) -> MainWorker {
        MainWorker::bootstrap_from_options(
            self.main_module.clone(),
            WorkerServiceOptions {
                module_loader: Rc::new(TypescriptModuleLoader::default()),
                permissions: self.permissions.clone(),
                blob_store: Default::default(),
                broadcast_channel: Default::default(),
                feature_checker: Default::default(),
                node_services: Default::default(),
                npm_process_state_provider: Default::default(),
                root_cert_store_provider: Default::default(),
                fetch_dns_resolver: Default::default(),
                shared_array_buffer_store: Default::default(),
                compiled_wasm_module_store: Default::default(),
                v8_code_cache: Default::default(),
                fs: self.fs.clone(),
            },
            WorkerOptions {
                bootstrap: BootstrapOptions {
                    cpu_count: std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1),
                    ..Default::default()
                },
                startup_snapshot: Some(WORKER_SNAPSHOT),
                create_params: create_isolate_create_params(),
                ..Default::default()
            },
        )
    }

    pub async fn eval_file<T>(&mut self) -> Result<Module<T>, TsError>
    where
        T: serde::de::DeserializeOwned,
    {
        let main_module = self.main_module.clone();

        let mut worker = self.worker.take().unwrap_or_else(|| self.create_worker());

        let future = async move {
            let mod_id = worker
                .preload_main_module(&main_module)
                .await
                .map_err(|e| TsError::Execution(format!("Failed to preload main module: {}", e)))?;
            worker
                .execute_main_module(&main_module)
                .await
                .map_err(|e| TsError::Execution(e.to_string()))?;
            worker.run_event_loop(false).await.map_err(|e| TsError::Execution(e.to_string()))?;

            let res = worker
                .js_runtime
                .get_module_namespace(mod_id)
                .map_err(|e| TsError::Execution(e.to_string()));

            let result = match res {
                Ok(global) => {
                    let scope = &mut worker.js_runtime.handle_scope();
                    let local = v8::Local::new(scope, global);
                    // Deserialize a `v8` object into a Rust type using `serde_v8`,
                    serde_v8::from_v8::<Module<T>>(scope, local.into())
                        .map_err(|e| TsError::Execution(e.to_string()))
                }
                Err(e) => Err(TsError::Execution(e.to_string())),
            }?;
            Ok::<_, TsError>(result)
        };

        let module = future.await?;

        Ok(module)
    }

    pub async fn eval_module<T>(&mut self) -> Result<T, TsError>
    where
        T: serde::de::DeserializeOwned + Clone,
    {
        self.eval_file().await.map(|m: Module<T>| (*m.default()).clone())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TsError {
    #[error("Failed to execute TypeScript: {0}")]
    Execution(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(String),
    #[error("Transpile error: {0}")]
    TranspileError(String),
}

#[derive(Debug)]
pub struct Module<T>
where
    T: serde::de::DeserializeOwned,
{
    pub default: T,
}

impl<'de, T> serde::Deserialize<'de> for Module<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper<T> {
            default: T,
        }

        let wrapper = Wrapper::deserialize(deserializer)?;
        Ok(Module { default: wrapper.default })
    }
}

impl<T> Module<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn default(&self) -> &T {
        &self.default
    }
}

pub fn create_isolate_create_params() -> Option<v8::CreateParams> {
    let maybe_mem_info = deno_runtime::sys_info::mem_info();
    maybe_mem_info.map(|mem_info| {
        v8::CreateParams::default().heap_limits_from_system_memory(mem_info.total, 0)
    })
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::Context;
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, Clone)]
    struct AppConfig {
        #[serde(rename = "databaseUrl")]
        database_url: String,
        #[serde(rename = "port")]
        port: u16,
        #[serde(rename = "features")]
        features: Vec<String>,
    }

    #[tokio::test]
    async fn test_typescript_config() -> Result<(), TsError> {
        let ts_content = r#"
        interface Config {
            databaseUrl: string;
            port: number;
            features: string[];
        }

        const config: Config = {
            databaseUrl: "postgres://localhost:5432/mydb",
            port: 8080,
            features: ["auth", "api"]
        };

        export default config;
        "#;

        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all(ts_content.as_bytes())?;

        let script_url = resolve_path(
            temp_file.path().as_ref() as &std::path::Path,
            &std::env::current_dir()
                .context("Unable to get CWD")
                .map_err(|e| TsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?,
        )
        .map_err(|e| TsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        let mut runner = TsRunner::new(script_url);

        let config: AppConfig = runner.eval_module().await?;

        assert_eq!(config.database_url, "postgres://localhost:5432/mydb");
        assert_eq!(config.port, 8080);
        assert_eq!(config.features, vec!["auth", "api"]);

        Ok(())
    }
}
