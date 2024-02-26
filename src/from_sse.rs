use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{PipelineData, Value, Span, ListStream};

pub struct Handler;

impl Handler {
    pub fn from_sse(
        &self,
        _call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = Span::unknown(); // Replace with actual Span if available
        
        let events = vec![
            Value::Record {
                cols: vec!["id".into(), "name".into(), "data".into()],
                vals: vec![
                    Value::String { val: "1".into(), span },
                    Value::String { val: "event_name_1".into(), span },
                    Value::String { val: "event_data_1".into(), span },
                ],
                span,
            },
            Value::Record {
                cols: vec!["id".into(), "name".into(), "data".into()],
                vals: vec![
                    Value::String { val: "2".into(), span },
                    Value::String { val: "event_name_2".into(), span },
                    Value::String { val: "event_data_2".into(), span },
                ],
                span,
            },
        ];

        Ok(PipelineData::ListStream(ListStream::from_iter(events.into_iter()), span))
    }
}