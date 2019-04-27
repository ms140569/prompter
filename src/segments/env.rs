use serde_json::{Value, Map};
use Segment;
use ResultSet;
use std::env;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct EnvSegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for EnvSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let fg = prompt.theme[SVN_CHANGES_FG];
        let bg = prompt.theme[SVN_CHANGES_BG];
        
        match &self.options {
            Some(val) => {
                match val["var"]{
                    serde_json::Value::String(ref value) => {
                        if let Ok(env_value) = env::var(&value) {
                            (format!(" {} ", env_value), fg, bg, String::new(), prompt.theme[RESET])
                        } else {
                            (String::from(" ENV:<not_found> "), fg, bg, String::new(), prompt.theme[RESET])
                        }
                        
                        },
                    _ => (String::from("Variable name not found"), fg, bg, String::new(), prompt.theme[RESET]),
                    }
            },
            None => (String::from("No option given"), fg, bg, String::new(), prompt.theme[RESET])
        }
    }
}

