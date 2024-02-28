use nu_plugin::{EvaluatedCall, LabeledError, StreamingPlugin};
use nu_protocol::{
    Category, ListStream, PipelineData, PluginExample, PluginSignature, Span, Type, Value,
};

use crate::parser;

pub struct Plugin;

impl StreamingPlugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        let span = Span::unknown();
        vec![PluginSignature::build("from sse")
            .usage("Converts an HTTP SSE (Server-Sent Events) stream into structured records")
            .search_terms(vec![
                "sse".to_string(),
                "stream".to_string(),
                "http".to_string(),
            ])
            .category(Category::Experimental)
            .input_output_types(vec![
                (Type::ListStream, Type::ListStream),
                (Type::String, Type::ListStream),
            ])
            .plugin_examples(vec![PluginExample {
                example: "http get http://example.com/events | from sse".to_string(),
                description:
                    "Converts an HTTP SSE (Server-Sent Events) stream into structured records"
                        .to_string(),
                result: Some(Value::record(
                        parser::Event::new(
                            Some("1"),
                            Some("creatureAlert"),
                            r#"{"id":"dra789","type":"Dragon","lat":45.4255,"lon":-75.6991,"urgency":"critical","desc":"Trapped by fallen trees after a storm."}"#,
                       ).to_record(span), span)),
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
            command_from_sse(call, input)
        } else {
            Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "The signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            })
        }
    }
}

fn command_from_sse(
    _call: &EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let mut parser = parser::Parser::new();

    let stream = input.into_iter().flat_map(move |value| match value {
        Value::String { val, internal_span } => {
            let events = parser.push(&val);
            events
                .into_iter()
                .map(move |event| Value::record(event.to_record(internal_span), internal_span))
        }
        _ => panic!("Value is not a String"),
    });

    let list_stream = ListStream::from_stream(stream, None);
    Ok(PipelineData::ListStream(list_stream, None))
}
