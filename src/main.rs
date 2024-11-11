use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_from_sse::FromPlugin;

fn main() {
    serve_plugin(&FromPlugin {}, MsgPackSerializer {})
}
