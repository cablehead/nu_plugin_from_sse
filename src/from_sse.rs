use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{ListStream, PipelineData, Record, ShellError, Span, Value};

pub struct Handler;

impl Handler {
    pub fn from_sse(
        &self,
        _call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = Span::unknown();

        let record = match self.create_record(
            vec!["id", "name", "data"],
            vec!["1", "event_name_1", "event_data_1"],
            span,
        ) {
            Ok(record) => record,
            Err(_) => {
                return Err(LabeledError {
                    label: "Error creating record".into(),
                    msg: "Failed to create record from columns and values".into(),
                    span: Some(span),
                });
            }
        };

        #[derive(Clone, Default, Debug)]
        struct Event {
            data: String,
            name: Option<String>,
            id: Option<String>,
        }

        impl Event {
            fn new() -> Self {
                Event::default()
            }

            fn reset(&mut self) {
                *self = Event::new();
            }

            fn is_empty(&self) -> bool {
                self.data.is_empty() && self.name.is_none() && self.id.is_none()
            }

            fn parse_line(&mut self, line: &str) -> Option<Self> {
                if line.trim().is_empty() {
                    if !self.is_empty() {
                        let cloned_event = self.clone();
                        self.reset();
                        return Some(cloned_event);
                    }
                } else if let Some(value) = line.strip_prefix("data:") {
                    self.data.push_str(value.trim());
                    self.data.push('\n');
                } else if let Some(value) = line.strip_prefix("event:") {
                    self.name = Some(value.trim().to_string());
                } else if let Some(value) = line.strip_prefix("id:") {
                    self.id = Some(value.trim().to_string());
                }
                None
            }
        }

        let mut event = Event::new();
        let mut remaining = String::new();

        let stream = input.into_iter().map(move |value| {
            match value {
                Value::String { val, .. } => {
                    remaining.push_str(&val);
                    while let Some(pos) = remaining.find('\n') {
                        let mut line = remaining.split_off(pos + 1);
                        std::mem::swap(&mut remaining, &mut line);
                        if let Some(emit) = event.parse_line(&line) {
                            eprintln!("WHA? {:?}", &emit);
                        }
                    }
                }
                _ => panic!("Value is not a String"),
            }
            record.clone()
        });

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
