use serde_json::{Value, Map};
use Segment;
use themes::*;
use ResultSet;
use prompt::Prompt;

#[derive(Debug)]
pub struct _DummySegment {
    pub options: Option<Map<String, Value>>,
    pub name: String,
}

impl Segment for _DummySegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {
        return (self.name.clone(), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
    }
}

