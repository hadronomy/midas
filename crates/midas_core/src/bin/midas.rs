use anyhow::Context;
use clap::{Arg, Command};
use deno_runtime::deno_core::resolve_path;
use midas_core::typescript::{TsError, TsRunner};
use miette::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub main: String,
    pub scripts: std::collections::HashMap<String, String>,
    pub dependencies: std::collections::HashMap<String, String>,
    pub dev_dependencies: std::collections::HashMap<String, String>,
    pub peer_dependencies: std::collections::HashMap<String, String>,
    pub optional_dependencies: std::collections::HashMap<String, String>,
    pub engines: std::collections::HashMap<String, String>,
    pub os: Vec<String>,
    pub cpu: Vec<String>,
    pub private: bool,
    pub workspaces: Vec<String>,
    pub publish_config: std::collections::HashMap<String, String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    human_panic::setup_panic!();
    let app = Command::new("mise")
        .version("0.1.0")
        .author("The MisE Team")
        .about("MisE is a tool for running JavaScript and TypeScript code in Rust")
        .arg(Arg::new("script").help("The script to run").required(true).index(1));

    let matches = app.get_matches();

    let script = matches.get_one::<String>("script").unwrap();
    let script_abs_path = std::fs::canonicalize(script).into_diagnostic()?;

    let script_url = resolve_path(
        script_abs_path.as_ref() as &std::path::Path,
        &std::env::current_dir()
            .context("Unable to get CWD")
            .map_err(|e| TsError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))
            .into_diagnostic()?,
    )
    .into_diagnostic()?;

    let mut runner = TsRunner::new(script_url);
    let module: AppConfig = runner.eval_module().await.into_diagnostic()?;

    println!("{}", serde_json::to_string_pretty(&module).unwrap());
    Ok(())
}
