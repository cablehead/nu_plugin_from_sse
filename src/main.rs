use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_from_sse::Plugin;

fn main() {
    serve_plugin(&mut Plugin {}, MsgPackSerializer {})
}
