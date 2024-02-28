use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{ListStream, PipelineData, Value};

use crate::parser;

pub struct Handler;

impl Handler {
    pub fn from_sse(
        &self,
        _call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let mut parser = parser::Parser::new();

        let stream = input.into_iter().flat_map(move |value| match value {
            Value::String { val, internal_span } => {
                let events = parser.push(&val);
                events
                    .into_iter()
                    .map(move |event| {
                        // Convert the Record to a Value::Record variant
                        Value::Record {
                            val: event.to_record(internal_span.clone()),
                            internal_span: internal_span.clone(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            }
            _ => panic!("Value is not a String"),
        });

        let list_stream = ListStream::from_stream(stream, None);
        Ok(PipelineData::ListStream(list_stream, None))
    }
}
