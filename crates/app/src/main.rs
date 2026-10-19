//! The lifefs app. With no subcommand it opens the window. `lifefs sweep` runs one sweep and
//! exits, so the LaunchAgent and the window share one binary.

// Release builds on Windows would otherwise open a console window next to the app window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod sweep;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Read every configured source once, update the store and the vault, then exit.
    Sweep,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stderr)
        .init();
    let cli = Cli::parse();
    let outcome = match cli.command {
        Some(Command::Sweep) => {
            sweep::run();
            Ok(())
        }
        None => run_window(),
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!(error = %error, "lifefs stopped");
            ExitCode::FAILURE
        }
    }
}

fn run_window() -> Result<(), Box<dyn std::error::Error>> {
    let commands = commands::builder();
    // Debug builds rewrite the TypeScript bindings, so the UI type-checks against the commands.
    #[cfg(debug_assertions)]
    commands.export(
        specta_typescript::Typescript::default(),
        concat!(env!("CARGO_MANIFEST_DIR"), "/ui/src/bindings.ts"),
    )?;
    tauri::Builder::default()
        .invoke_handler(commands.invoke_handler())
        .run(tauri::generate_context!())?;
    Ok(())
}
