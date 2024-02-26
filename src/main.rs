use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_from_sse::Example;

fn main() {
    serve_plugin(&mut Example {}, MsgPackSerializer {})
}
