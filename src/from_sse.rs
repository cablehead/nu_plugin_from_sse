use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{ListStream, PipelineData, Record, ShellError, Span, Value};

pub struct Handler;

impl Handler {
    pub fn from_sse(
        &self,
        _call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = Span::unknown(); // Replace with actual Span if available

        // Pre-create the record outside of the closure to avoid capturing `self`
        let record = match self.create_record(
            vec!["id", "name", "data"],
            vec!["1", "event_name_1", "event_data_1"],
            span,
        ) {
            Ok(record) => record,
            Err(_) => return Err(LabeledError { // Handle error appropriately
                label: "Error creating record".into(),
                msg: "Failed to create record from columns and values".into(),
                span: Some(span),
            }),
        };

        // Use a clone of the pre-created record for each item in the input
        let stream = input.into_iter().map(move |_value| record.clone());

        let list_stream = ListStream::from_stream(stream, None);
        Ok(PipelineData::ListStream(list_stream, None))
    }

    fn create_record(
        &self,
        cols: Vec<&str>,
        vals: Vec<&str>,
        span: Span,
    ) -> Result<Value, LabeledError> {
        let cols = cols.into_iter().map(String::from).collect::<Vec<String>>();
        let vals = vals
            .into_iter()
            .map(|val| Value::string(val, span))
            .collect::<Vec<Value>>();
        Record::from_raw_cols_vals(cols, vals, span, Span::unknown())
            .map_err(|err| self.handle_error(err, span))
            .map(|rec| Value::Record {
                val: rec,
                internal_span: span,
            })
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
