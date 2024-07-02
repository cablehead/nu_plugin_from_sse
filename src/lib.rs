use nu_plugin::{Plugin, PluginCommand};

mod commands;
mod parser;
mod plugin;

pub use commands::SSE;
pub use plugin::FromPlugin;

impl Plugin for FromPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(SSE)]
    }
}
