use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_from_sse::Handler;

fn main() {
    serve_plugin(&mut Handler {}, MsgPackSerializer {})
}
