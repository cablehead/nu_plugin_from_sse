use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    record, Category, Example, LabeledError, ListStream, PipelineData, ShellError, Signature, Span,
    Type, Value, Signals,
};
use std::sync::{atomic::AtomicBool, Arc};

use crate::parser::{Event, Parser};
use crate::plugin;

pub struct SSE;

impl PluginCommand for SSE {
    type Plugin = plugin::FromPlugin;
    fn name(&self) -> &str {
        "from sse"
    }

    fn description(&self) -> &str {
        "Converts an HTTP SSE (Server-Sent Events) stream into structured records"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["sse", "stream", "http"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "http get http://example.com/events | from sse",
            description: "Converts an HTTP SSE (Server-Sent Events) stream into structured records",
            result: Some(event_to_record_value(
                &Event::new(
                    Some("1"),
                    Some("creatureAlert"),
                    r#"{"id":"dra789","type":"Dragon","lat":45.4255,"lon":-75.6991,"urgency":"critical","desc":"Trapped by fallen trees after a storm."}"#,
                ),
                Span::unknown(),
            )),
        }]
    }

    fn signature(&self) -> Signature {
        Signature::build("from sse")
            .category(Category::Experimental)
            .input_output_types(vec![
                (Type::List(Box::new(Type::Any)), Type::List(Box::new(Type::Any))),
                (Type::String, Type::List(Box::new(Type::Any))),
            ])
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let mut parser = Parser::new();
        let stream = input
            .into_iter()
            .flat_map(move |value| process_value(value, &mut parser));
        let list_stream = ListStream::new(stream, span, Signals::new(Arc::new(AtomicBool::new(false))));
        Ok(PipelineData::ListStream(list_stream, None))
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

fn process_string(s: &str, span: Span, parser: &mut Parser) -> Vec<Value> {
    let events = parser.push(s);
    events
        .into_iter()
        .map(move |event| event_to_record_value(&event, span))
        .collect()
}

fn process_error(span: Span) -> Vec<Value> {
    vec![Value::error(
        ShellError::TypeMismatch {
            err_message: "Value is not a String".into(),
            span,
        },
        span,
    )]
}

fn process_value(value: Value, parser: &mut Parser) -> impl Iterator<Item = Value> {
    let span = value.span();
    match value {
        Value::String { val, .. } => process_string(&val, span, parser),
        _ => process_error(span),
    }
    .into_iter()
}
