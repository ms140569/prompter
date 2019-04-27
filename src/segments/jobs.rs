use serde_json::{Value, Map};
use Segment;
use ResultSet;
use std::process::Command;
use themes::*;
use std::os::unix::process::parent_id;
use std::str;
use prompt::Prompt;

pub struct JobsSegment{
    pub options: Option<Map<String, Value>>
}

impl Segment for JobsSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let ppid = parent_id();

        let output = Command::new("ps")
            .arg("-a")
            .arg("-o")
            .arg("ppid")            
            .output()
            .expect("Failed to execute git.");

        let std_out_value = match str::from_utf8(&output.stdout){
                Ok(val) => val,
                Err(_) => "",
            };
        
        let mut counter = 0;
        
        for line in std_out_value.lines() {
            if let Ok(val) = line.parse::<u32>() {
                if val == ppid {
                    counter += 1;
                }
            } 
        }

        // do not count ourselfs ...
        counter -= 1;

        if counter > 0 {
            return (String::from(format!(" {} ", counter )), prompt.theme[JOBS_FG], prompt.theme[JOBS_BG], String::new(), prompt.theme[RESET]);
        } else {
            return (String::from(""), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
        }
    }
}

