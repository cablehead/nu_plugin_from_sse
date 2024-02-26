use crate::from_sse::Handler; // Assume renaming Example to Handler in from_sse module
use nu_plugin::{EvaluatedCall, LabeledError, StreamingPlugin};
use nu_protocol::{
    Category, PipelineData, PluginExample, PluginSignature, Span, Type, Value,
};

impl StreamingPlugin for Handler {
    fn signature(&self) -> Vec<PluginSignature> {
        let span = Span::unknown();
        vec![
            PluginSignature::build("from sse") // Updated to "from sse"
                .usage("Converts HTTP SSE (Server-Sent Events) into structured events")
                .search_terms(vec!["sse".into(), "stream".into(), "http".into()])
                .category(Category::Experimental)
                .input_output_types(vec![(Type::Nothing, Type::List(Type::Record(vec![
                    ("id".into(), Type::String.into()),
                    ("name".into(), Type::String.into()),
                    ("data".into(), Type::String.into()),
                ])))])
                .plugin_examples(vec![PluginExample {
                    example: "http get example.com/events | from sse".into(),
                    description: "Converts SSE from an HTTP GET request into structured events".into(),
                    result: None, // You might want to include a hypothetical output example here
                }])
        ]
    }

    fn run(
        &mut self,
        name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        if name == "from sse" { // Directly compare the name with "from sse"
            self.from_sse(call, input) // Ensure this method is implemented in Handler
        } else {
            Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "The signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            })
        }
    }
}
