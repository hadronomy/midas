use std::{env, path::PathBuf};

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        return;
    }

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let cli_snapshot_path = output.join("CLI_SNAPSHOT.bin");
    create_cli_snapshot(cli_snapshot_path);
}

fn create_cli_snapshot(snapshot_path: PathBuf) {
    use deno_runtime::ops::bootstrap::SnapshotOptions;

    let snapshot_options = SnapshotOptions {
        ts_version: ts::version(),
        v8_version: deno_core::v8::VERSION_STRING,
        target: std::env::var("TARGET").unwrap(),
    };

    deno_runtime::snapshot::create_runtime_snapshot(snapshot_path, snapshot_options, vec![]);
}

mod ts {
    pub(crate) fn version() -> String {
        let file_text = std::fs::read_to_string("tsc/00_typescript.js").unwrap();
        let version_text = " version = \"";
        for line in file_text.lines() {
            if let Some(index) = line.find(version_text) {
                let remaining_line = &line[index + version_text.len()..];
                return remaining_line[..remaining_line.find('"').unwrap()].to_string();
            }
        }
        panic!("Could not find ts version.")
    }
}
