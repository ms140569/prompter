use serde_json::{Value, Map};
use Segment;
use ResultSet;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct UsernameSegment {
    pub options: Option<Map<String, Value>>
}


impl Segment for UsernameSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let mut bgcolor = prompt.theme[USERNAME_BG];
        
        if whoami::username() == "root" {
            bgcolor = prompt.theme[USERNAME_ROOT_BG];
        }
        return (String::from(" \\u "), prompt.theme[USERNAME_FG], bgcolor, String::new(), prompt.theme[RESET]);
    }
}


