pub mod console;
use bevy::prelude::*;
use bevy_screen_diags::ScreenDiagsTextPlugin;


pub mod prelude {
    pub use crate::PMDevToolsPlugin;
    pub use bevy_console::{AddConsoleCommand, ConsoleCommand};
    pub use clap;
    pub use bevy_console;
    pub use clap::Parser;
}

/// A set of tools targeted for ease of development.
pub struct PMDevToolsPlugin;
impl Plugin for PMDevToolsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(console::PMConsolePlugin)
            .add_plugins(ScreenDiagsTextPlugin);
    }
}
