use serde_json::{Value, Map};
use Segment;
use ResultSet;
use std::process::Command;
use themes::*;
use std::str;
use prompt::Prompt;

pub struct StdoutSegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for StdoutSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {
        if let Some(ref options) = self.options {
            if let Value::Array(ref arr) = options["command"] {
                if arr.len() > 0 {
                    if let Value::String(binary) = &arr[0] {
                        let mut cmd = Command::new(binary);

                        for arg in &arr[1..] {
                            if let Value::String(a) = arg {
                                cmd.arg(a);
                            }
                        }

                        match cmd.output() {
                            Err(_) => {
                                eprintln!("Failed to execute command: {}", binary);
                                return (String::new(), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
                            },
                            Ok(output) => {
                                let std_out_value = match str::from_utf8(&output.stdout){
                                    Ok(val) => val,
                                    Err(_) => "",
                                };

                                
                                return (format!(" {} ", String::from(std_out_value.trim_end())), prompt.theme[PATH_FG], prompt.theme[PATH_BG], String::new(), prompt.theme[RESET]);
                            }
                        }
                    }
                }
            }
        }
        return (String::new(), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
    }
}

