use nu_protocol::{record, Span, Value};

#[derive(Clone, Default, Debug)]
pub struct Event {
    pub id: Option<String>,
    pub name: Option<String>,
    pub data: String,
}

impl Event {
    pub fn new(id: Option<&str>, name: Option<&str>, data: &str) -> Self {
        Event {
            id: id.map(str::to_string),
            name: name.map(str::to_string),
            data: data.into(),
        }
    }

    fn reset(&mut self) {
        *self = Event::default();
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty() && self.name.is_none() && self.id.is_none()
    }

    pub fn to_record_value(&self, span: Span) -> nu_protocol::Value {
        Value::record(
            record! {
                "id" => match &self.id {
                    Some(id) => Value::string(id.clone(), span),
                    None => Value::nothing(span),
                },
                "name" => match &self.name {
                    Some(name) => Value::string(name.clone(), span),
                    None => Value::nothing(span),
                },
                "data" => Value::string(self.data.clone(), span),
            },
            span,
        )
    }
}

pub struct Parser {
    event: Event,
    remaining: String,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            event: Event::default(),
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
