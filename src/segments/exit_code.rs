use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct ExitCodeSegment{
    pub options: Option<Map<String, Value>>,
    pub prev_error:i32
}

impl Segment for ExitCodeSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {
        if self.prev_error == 0 {
            return (String::from(""), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
        } else {
            return (format!(" {} ", self.prev_error), prompt.theme[CMD_FAILED_FG], prompt.theme[CMD_FAILED_BG], String::new(), prompt.theme[RESET]);
        }
        
    }
}

