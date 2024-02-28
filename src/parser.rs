use nu_protocol::{record, Span, Value};

#[derive(Clone, Default, Debug)]
pub struct Event {
    id: Option<String>,
    name: Option<String>,
    data: String,
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

    pub fn to_record(&self, internal_span: Span) -> nu_protocol::Record {
        record! {
            "id" => match &self.id {
                Some(id) => Value::String {
                    val: id.clone(),
                    internal_span: internal_span.clone(),
                },
                None => Value::Nothing {
                    internal_span: internal_span.clone(),
                },
            },
            "name" => match &self.name {
                Some(name) => Value::String {
                    val: name.clone(),
                    internal_span: internal_span.clone(),
                },
                None => Value::Nothing {
                    internal_span: internal_span.clone(),
                },
            },
            "data" => Value::String {
                val: self.data.clone(),
                internal_span: internal_span,
            }
        }
    }
}

pub struct Parser {
    event: Event,
    remaining: String,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            event: Event::new(),
            remaining: String::new(),
        }
    }

    pub fn push(&mut self, input: &str) -> Vec<Event> {
        self.remaining.push_str(input);
        let mut ret = Vec::new();
        while let Some(pos) = self.remaining.find('\n') {
            let mut line = self.remaining.split_off(pos + 1);
            std::mem::swap(&mut self.remaining, &mut line);
            if let Some(event) = self.parse_line(&line) {
                ret.push(event);
            }
        }
        ret
    }

    fn parse_line(&mut self, line: &str) -> Option<Event> {
        if line.trim().is_empty() {
            if !self.event.is_empty() {
                let cloned_event = self.event.clone();
                self.event.reset();
                return Some(cloned_event);
            }
        } else if let Some(value) = line.strip_prefix("data:") {
            self.event.data.push_str(value.trim());
            self.event.data.push('\n');
        } else if let Some(value) = line.strip_prefix("event:") {
            self.event.name = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("id:") {
            self.event.id = Some(value.trim().to_string());
        }
        None
    }
}
