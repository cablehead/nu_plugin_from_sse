use nu_plugin::{EvaluatedCall, LabeledError, StreamingPlugin};
use nu_protocol::{
    record, Category, ListStream, PipelineData, PluginExample, PluginSignature, ShellError, Span,
    Type, Value,
};

use crate::parser::{Event, Parser};

pub struct Plugin;

impl StreamingPlugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
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
                result: Some(
                    event_to_record_value(
                        &Event::new(
                            Some("1"),
                            Some("creatureAlert"),
                            r#"{"id":"dra789","type":"Dragon","lat":45.4255,"lon":-75.6991,"urgency":"critical","desc":"Trapped by fallen trees after a storm."}"#,)
                       ,Span::unknown())),
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

fn event_to_record_value(event: &Event, span: Span) -> nu_protocol::Value {
    Value::record(
        record! {
            "id" => match &event.id {
                Some(id) => Value::string(id.clone(), span),
                None => Value::nothing(span),
            },
            "name" => match &event.name {
                Some(name) => Value::string(name.clone(), span),
                None => Value::nothing(span),
            },
            "data" => Value::string(event.data.clone(), span),
        },
        span,
    )
}

fn process_string(s: &str, span: Span, parser: &mut Parser) -> impl Iterator<Item = Value> {
    let events = parser.push(s);
    events
        .into_iter()
        .map(move |event| event_to_record_value(&event, span))
}

fn process_error(span: Span) -> impl Iterator<Item = Value> {
    std::iter::once(Value::error(
        ShellError::TypeMismatch {
            err_message: "Value is not a String".into(),
            span,
        },
        span,
    ))
}

use itertools::Either;

fn process_value(value: Value, parser: &mut Parser) -> impl Iterator<Item = Value> {
    let span = value.span();
    match value {
        Value::String { val, .. } => Either::Left(process_string(&val, span, parser)),
        _ => Either::Right(process_error(span)),
    }
}

fn command_from_sse(
    _call: &EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError> {
    let mut parser = Parser::new();

    let stream = input
        .into_iter()
        .flat_map(move |value| process_value(value, &mut parser));

    let list_stream = ListStream::from_stream(stream, None);
    Ok(PipelineData::ListStream(list_stream, None))
}
