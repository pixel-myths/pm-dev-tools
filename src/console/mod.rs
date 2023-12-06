use std::str::FromStr;

use bevy::prelude::{App, Plugin, ResMut};
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use bevy_screen_diags::ScreenDiagsState;
use clap::Parser;

/// Settings about various diagnostics options
#[derive(Parser, ConsoleCommand)]
#[command(name = "diagnostics")]
struct DiagnosticsCommand {
    action: DiagnosticAction,
}

#[derive(clap::ValueEnum, Clone, PartialEq, Eq)]
enum DiagnosticAction {
    Fps,
}

pub enum PossibleDiagnosticActionsParseError {
    InvalidActionName,
}

impl FromStr for DiagnosticAction {
    type Err = PossibleDiagnosticActionsParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "fps" => Ok(Self::Fps),
            _ => Err(PossibleDiagnosticActionsParseError::InvalidActionName),
        }
    }
}

fn diagnostics_command(
    mut log: ConsoleCommand<DiagnosticsCommand>,
    mut state: ResMut<ScreenDiagsState>,
) {
    if let Some(Ok(DiagnosticsCommand { action })) = log.take() {
        if action == DiagnosticAction::Fps {
            if state.enabled() {
                state.disable();
            } else {
                state.enable();
            }
        }
    }
}

pub struct PMConsolePlugin;

impl Plugin for PMConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsolePlugin)
            .add_console_command::<DiagnosticsCommand, _>(diagnostics_command);
    }
}
