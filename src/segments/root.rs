use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct RootSegment{
    pub options: Option<Map<String, Value>>,
    pub prev_error:i32
}

impl Segment for RootSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {
        let mut fg = prompt.theme[CMD_PASSED_FG];
        let mut bg = prompt.theme[CMD_PASSED_BG];

        if self.prev_error != 0 {
            fg = prompt.theme[CMD_FAILED_FG];
            bg = prompt.theme[CMD_FAILED_BG];
        }
        
        return (String::from(" \\$ "), fg, bg, String::new(), prompt.theme[RESET]);
    }
}

