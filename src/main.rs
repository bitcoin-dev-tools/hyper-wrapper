use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct ParameterList {
    name: String,
    values: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct HyperfineConfig {
    #[serde(default)]
    command: String,
    #[serde(rename = "parameter-list")]
    #[serde(default)]
    parameter_lists: Vec<ParameterList>,
    #[serde(default)]
    prepare: Option<String>,
    #[serde(default)]
    cleanup: Option<String>,
    #[serde(default)]
    runs: Option<u32>,
    #[serde(default)]
    #[serde(rename = "show-output")]
    show_output: Option<bool>,
    #[serde(rename = "export-json")]
    #[serde(default)]
    export_json: Option<String>,
    #[serde(default)]
    warmup: Option<u32>,
    #[serde(rename = "min-runs")]
    #[serde(default)]
    min_runs: Option<u32>,
    #[serde(rename = "max-runs")]
    #[serde(default)]
    max_runs: Option<u32>,
}

fn build_hyperfine_command(config: &HyperfineConfig) -> Command {
    let mut cmd = Command::new("hyperfine");

    // Add parameter lists
    for param in &config.parameter_lists {
        cmd.arg("--parameter-list");
        cmd.arg(&param.name);
        cmd.arg(&param.values);
    }

    // Add prepare command
    if let Some(prepare) = &config.prepare {
        cmd.arg("--prepare");
        cmd.arg(prepare);
    }

    // Add cleanup command
    if let Some(cleanup) = &config.cleanup {
        cmd.arg("--cleanup");
        cmd.arg(cleanup);
    }

    // Add runs
    if let Some(runs) = config.runs {
        cmd.arg("--runs");
        cmd.arg(runs.to_string());
    }

    // Add show-output flag
    if config.show_output.unwrap_or(false) {
        cmd.arg("--show-output");
    }

    // Add export-json
    if let Some(export_json) = &config.export_json {
        cmd.arg("--export-json");
        cmd.arg(export_json);
    }

    // Add warmup
    if let Some(warmup) = config.warmup {
        cmd.arg("--warmup");
        cmd.arg(warmup.to_string());
    }

    // Add min-runs
    if let Some(min_runs) = config.min_runs {
        cmd.arg("--min-runs");
        cmd.arg(min_runs.to_string());
    }

    // Add max-runs
    if let Some(max_runs) = config.max_runs {
        cmd.arg("--max-runs");
        cmd.arg(max_runs.to_string());
    }

    // Add the main command
    cmd.arg(&config.command);

    cmd
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read config file path and commit hashes from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <config.json> <base_commit> <head_commit> <results_dir>",
            args[0]
        );
        std::process::exit(1);
    }

    let base_commit = &args[2];
    let head_commit = &args[3];
    let results_dir = &args[4];

    // Read and parse config file
    let config_content = fs::read_to_string(&args[1])?;
    let mut config: HyperfineConfig = serde_json::from_str(&config_content)?;

    // Add commit parameter list
    config.parameter_lists.push(ParameterList {
        name: "commit".to_string(),
        values: format!("{},{}", base_commit, head_commit),
    });

    config.export_json = Some(results_dir.to_string());

    // Build and execute hyperfine command
    let mut command = build_hyperfine_command(&config);

    println!("Executing command: {:?}", command);

    let status = command.status()?;

    if !status.success() {
        eprintln!("Hyperfine command failed with status: {}", status);
        std::process::exit(1);
    }

    Ok(())
}
