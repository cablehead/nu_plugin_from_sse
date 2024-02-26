use crate::from_sse::Handler;
use nu_plugin::{EvaluatedCall, LabeledError, StreamingPlugin};
use nu_protocol::{Category, PipelineData, PluginExample, PluginSignature, Type, Value};

impl StreamingPlugin for Handler {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("from sse")
            .usage("Converts HTTP SSE (Server-Sent Events) into structured events")
            .search_terms(vec![
                "sse".to_string(),
                "stream".to_string(),
                "http".to_string(),
            ])
            .category(Category::Experimental)
            .input_output_types(vec![(
                Type::ListStream,
                Type::List(Box::new(Type::Record(vec![
                    ("id".to_string(), Type::String),
                    ("name".to_string(), Type::String),
                    ("data".to_string(), Type::String),
                ]))),
            )])
            .plugin_examples(vec![PluginExample {
                example: "http get example.com/events | from sse".to_string(),
                description: "Converts SSE from an HTTP GET request into structured events"
                    .to_string(),
                result: None, // You might want to include a hypothetical output example here
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        if name == "from sse" {
            self.from_sse(call, input)
        } else {
            Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "The signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            })
        }
    }
}
