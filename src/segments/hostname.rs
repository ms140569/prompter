use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct HostnameSegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for HostnameSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {
        return (String::from(" \\h "), prompt.theme[HOSTNAME_FG], prompt.theme[HOSTNAME_BG], String::new(), prompt.theme[RESET]);
    }
}

