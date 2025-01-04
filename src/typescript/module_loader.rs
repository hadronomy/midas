use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceMapOption;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::resolve_import;
use deno_runtime::deno_core::ModuleLoadResponse;
use deno_runtime::deno_core::ModuleLoader;
use deno_runtime::deno_core::ModuleSource;
use deno_runtime::deno_core::ModuleSourceCode;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::deno_core::ModuleType;
use deno_runtime::deno_core::RequestedModuleType;
use deno_runtime::deno_core::ResolutionKind;

type SourceMapStore = Rc<RefCell<HashMap<String, Vec<u8>>>>;

#[derive(Default)]
pub struct TypescriptModuleLoader {
    pub(crate) source_maps: SourceMapStore,
}

impl ModuleLoader for TypescriptModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let source_maps = self.source_maps.clone();
        fn load(
            source_maps: SourceMapStore,
            module_specifier: &ModuleSpecifier,
        ) -> Result<ModuleSource, AnyError> {
            let path = module_specifier
                .to_file_path()
                .map_err(|_| anyhow!("Only file:// URLs are supported."))?;

            let media_type = MediaType::from_path(&path);
            let (module_type, should_transpile) = match MediaType::from_path(&path) {
                MediaType::JavaScript | MediaType::Mjs | MediaType::Cjs => {
                    (ModuleType::JavaScript, false)
                }
                MediaType::Jsx => (ModuleType::JavaScript, true),
                MediaType::TypeScript
                | MediaType::Mts
                | MediaType::Cts
                | MediaType::Dts
                | MediaType::Dmts
                | MediaType::Dcts
                | MediaType::Tsx => (ModuleType::JavaScript, true),
                MediaType::Json => (ModuleType::Json, false),
                _ => bail!("Unknown extension {:?}", path.extension()),
            };

            let code = std::fs::read_to_string(&path)?;
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;
                let res = parsed.transpile(
                    &deno_ast::TranspileOptions {
                        imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Remove,
                        use_decorators_proposal: true,
                        ..Default::default()
                    },
                    &deno_ast::TranspileModuleOptions::default(),
                    &deno_ast::EmitOptions {
                        source_map: SourceMapOption::Separate,
                        inline_sources: true,
                        ..Default::default()
                    },
                )?;
                let res = res.into_source();
                let source_map = res.source_map.ok_or(anyhow!("No source map"))?;
                source_maps
                    .borrow_mut()
                    .insert(module_specifier.to_string(), source_map.into_bytes());
                res.text
            } else {
                code
            };
            Ok(ModuleSource::new(
                module_type,
                ModuleSourceCode::String(code.into()),
                module_specifier,
                None,
            ))
        }

        ModuleLoadResponse::Sync(load(source_maps, module_specifier))
    }

    fn get_source_map(&self, specifier: &str) -> std::option::Option<std::vec::Vec<u8>> {
        self.source_maps.borrow().get(specifier).cloned()
    }
}
