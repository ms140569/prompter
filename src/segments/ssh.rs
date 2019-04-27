use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use std::env;
use prompt::Prompt;

#[derive(Debug)]
pub struct SshSegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for SshSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let mut result = String::from("");
        
        if env::var("SSH_CLIENT").is_ok() {
            result = format!(" {} ", prompt.symbols.network);
        }
        
        return (result, prompt.theme[SSH_FG], prompt.theme[SSH_BG], String::new(), prompt.theme[RESET]);
    }
}

