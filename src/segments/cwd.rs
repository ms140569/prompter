use serde_json::{Value, Map};
use Segment;
use ResultSet;
use ConfigMap;
use std::env;
use themes::*;
use prompt::Prompt;

#[derive(Debug)]
pub struct CwdSegment{
    pub options: Option<Map<String, Value>>,
    pub global_config: Option<ConfigMap>
}

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    Plain,
    DirOnly,
    Expand,
}
    

impl CwdSegment {

    const ELLIPSIS: &'static str = "\u{2026}";
    
    fn get_full_cwd(&self) -> bool {
        if let Some(ref gc) = self.global_config {
            if let Some(val) = gc.get("full_cwd") {
                match val {
                    Value::Bool(ref v) => { return *v; },
                    _ => {return false; }
                }
            }
            return false;
        }
        false
    }

    /* mode: If "plain", then simple text will be used to show the cwd. If "dironly", only the current directory will be shown. 
       Otherwise expands the cwd into individual directories.
     */

   
    fn get_mode(&self) -> Mode {
        if let Some(ref gc) = self.global_config {
            if let Some(val) = gc.get("mode") {
                match val {
                    Value::String(s) => {
                        match s.to_lowercase().as_str() {
                            "plain"   => { return Mode::Plain; },
                            "dironly" => { return Mode::DirOnly;},
                            _         => { return Mode::Expand;},                            
                        }
                    },
                    _ => {}
                }
            }
        }
        Mode::Expand
    }

    // we treat "unlimited" here as 0
    // Should this be a Some/None ?
    fn get_max_dir_size(&self) -> usize {
        self.get_number_or_default("max_dir_size", 0)
    }

    fn get_max_depth(&self) -> usize {
        self.get_number_or_default("max_depth", 5)
    }

    fn get_number_or_default(&self, key: &str, default_value: usize) -> usize {
        if let Some(ref gc) = self.global_config {
            if let Some(val) = gc.get(key) {
                match val {
                    Value::Number(num) => {
                        if let Some(ret_val) = num.as_u64() {
                            return ret_val as usize;
                        }
                    },
                    _ => { return default_value; }
                }
            }
        }
        default_value

    }

    fn replace_homedir(&self, path: &str) -> String {
        let mut homedir = String::new();

        if let Ok(home_var) = env::var("HOME") {
            homedir.push_str(&home_var);
        } else {
            eprintln!("HOME variable not set. Something is very broken here.");
            return String::from(path);
        }
        
        if path.starts_with(homedir.as_str()) {
            let len = homedir.len();
            let mut ret_val = String::from("~");
            ret_val.push_str(&path[len..]);
            return ret_val;
        }
        String::from(path)
    }

    fn get_fg_bg(&self, name: &str, theme: &Theme, is_last_dir: bool, home_special_display: bool) -> (Colorcode, Colorcode, bool) {
        if self.requires_special_home_display(home_special_display, name) {
            return (theme[HOME_FG], theme[HOME_BG], true);
        }

        if is_last_dir {
            return (theme[CWD_FG], theme[PATH_BG], false);
        } else {
            return (theme[PATH_FG], theme[PATH_BG], false);
        }
    }

    fn requires_special_home_display(&self, home_special_display: bool, name: &str) -> bool {
        return name == "~" && home_special_display;
    }

    fn maybe_shorten_name(&self, name: &str, max_dir_size: usize) -> String {
        
        if (max_dir_size > 0) && (max_dir_size < name.len() && name != CwdSegment::ELLIPSIS) {
            return String::from(&name[..max_dir_size]);
        } else {
            return String::from(name);
        }
    }
    
    fn get_cwd(&self) -> Vec<String> {

        let pwd;
        
        if let Ok(env_value) = env::var("PWD") {
            pwd = env_value;
        } else {
            match env::current_dir() {
                Err(err) => {
                    eprintln!("Could not get current directory: {}", err);
                    return Vec::new(); 
                },
                Ok(p) => {
                    pwd = p.display().to_string();
                }
            }
        }

        let mut can_path: String = self.replace_homedir(&pwd);

        if can_path.starts_with("/") {
            can_path.remove(0);
        }

        return can_path.split("/").map(|s| s.to_string()).collect();
    }
}

impl Segment for CwdSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet {

        let home_special_display = prompt.theme[HOME_SPECIAL_DISPLAY] == 1;

        let mut names = self.get_cwd();
        
        let mut cwd_result_string = String::new();
        let full_cwd = self.get_full_cwd();
        let mode = self.get_mode();
        let max_depth = self.get_max_depth();
        let max_dir_size = self.get_max_dir_size();

        if max_depth < 1 {
            eprintln!("max_depth ought to be greater than zero. Ignoring.");
        } else if names.len() > max_depth {
            let n_before = if max_depth > 2 { 2 } else {max_depth - 1}; 
            let mut new_names: Vec<String> = Vec::new();

            new_names.extend_from_slice(&names[..n_before]);
            new_names.push(CwdSegment::ELLIPSIS.to_string());
            new_names.extend_from_slice(&names[(names.len() - (n_before + 1)) ..]);
            
            names = new_names;
        }

        if mode == Mode::DirOnly {
            names = names[names.len()-1 ..].to_vec();
        } else if mode == Mode::Plain {
            cwd_result_string = names.join("/");
            return (format!(" {} ", cwd_result_string), prompt.theme[CWD_FG], prompt.theme[PATH_BG], String::new(), prompt.theme[RESET]);
        }

        let last_idx = names.len() -1;
        let mut homedir_found = false; 
        
        // do the full monty ...
        for (idx, name) in names.iter().enumerate() {
            let is_last_dir = idx == last_idx;
            let ( fg, bg, homedir_included ) = self.get_fg_bg(name, &prompt.theme, is_last_dir, home_special_display); 

            if homedir_included {
                homedir_found = true;
            }
            
            let mut shorty = name.to_string();

            if ! ( is_last_dir && full_cwd ) {
                shorty = self.maybe_shorten_name(&name, max_dir_size);
            } 

            if idx != 0 {

                if idx == 1 && homedir_found {
                    cwd_result_string.push_str(&painter::bgcolor(prompt.theme[PATH_BG]));            
                    cwd_result_string.push_str(&painter::fgcolor(prompt.theme[HOME_BG]));
                    cwd_result_string.push_str(&format!("{}", prompt.symbols.separator));
                } else {
                    cwd_result_string.push_str(&painter::fgcolor(prompt.theme[SEPARATOR_FG]));
                    cwd_result_string.push_str(&format!("{}", prompt.symbols.separator_thin));
                }
            }
            
            cwd_result_string.push_str(&painter::bgcolor(bg));            
            cwd_result_string.push_str(&painter::fgcolor(fg));
            
            cwd_result_string.push_str(&format!(" {} ", &shorty));
        }

        let downstream = if homedir_found && names.len() == 1 {prompt.theme[HOME_BG] } else {prompt.theme[PATH_BG]};
        
        let fg = if homedir_found {prompt.theme[HOME_FG] } else {prompt.theme[CWD_FG]};
        let bg = if homedir_found {prompt.theme[HOME_BG] } else {prompt.theme[PATH_BG]};
        
        return (format!("{}", cwd_result_string), fg, bg, String::new(), downstream);
    }
}
