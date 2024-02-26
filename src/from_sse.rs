use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{PipelineData, Value, Span, ListStream, Record, ShellError};

pub struct Handler;

impl Handler {
    pub fn from_sse(
        &self,
        _call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = Span::unknown(); // Replace with actual Span if available

        // Construct records for demonstration
        let event1 = self.create_record(vec!["id", "name", "data"], vec!["1", "event_name_1", "event_data_1"], span)?;
        let event2 = self.create_record(vec!["id", "name", "data"], vec!["2", "event_name_2", "event_data_2"], span)?;

        let events = vec![event1, event2];

        // Use ListStream::from_stream
        let stream = ListStream::from_stream(events.into_iter(), None);
        Ok(PipelineData::ListStream(stream, None))
    }

    fn create_record(&self, cols: Vec<&str>, vals: Vec<&str>, span: Span) -> Result<Value, LabeledError> {
        let cols = cols.into_iter().map(String::from).collect::<Vec<String>>();
        let vals = vals.into_iter().map(|val| Value::string(val, span)).collect::<Vec<Value>>();
        Record::from_raw_cols_vals(cols, vals, span, Span::unknown())
            .map_err(|err| self.handle_error(err, span))
            .map(|rec| Value::Record { val: rec, internal_span: span })
    }

    fn handle_error(&self, _err: ShellError, span: Span) -> LabeledError {
        // Customize error handling as needed
        LabeledError {
            label: "Error creating record".into(),
            msg: "Failed to create record from columns and values".into(),
            span: Some(span),
        }
    }
}
