extern crate serde_json;
extern crate whoami;
extern crate regex;

use std::fs;
use std::env;
use serde_json::{Value, Map};
use std::collections::HashMap;
use std::process;
use std::path::Path;

mod segments;
mod themes;
mod prompt;
mod constants;

use segments::env::EnvSegment;
use segments::cwd::CwdSegment;
use segments::username::UsernameSegment;
use segments::git::GitSegment;
use segments::hostname::HostnameSegment;
use segments::virtual_env::VirtualEnvSegment;
use segments::read_only::ReadOnlySegment;
use segments::ssh::SshSegment;
use segments::exit_code::ExitCodeSegment;
use segments::stdout::StdoutSegment;
use segments::jobs::JobsSegment;
use segments::root::RootSegment;

use themes::*;
use themes::default::default_theme;
use themes::painter;

use prompt::Prompt;

//                name  , FG       , BG       , SEP   , SEG col
type ResultSet = (String, Colorcode, Colorcode, String, Colorcode);
type ConfigMap = HashMap<String, Value>;

pub trait Segment {
    fn compute(&self, &Prompt) -> ResultSet;
}

fn get_segment_for_string(s: String, opt: Option<Map<String, Value>>, prev_error: i32, global_config: Option<ConfigMap>) -> Result<Box<dyn Segment>, String> {
    match s.to_lowercase().as_ref() {
        
        "hostname"    => Ok(Box::new(HostnameSegment {options: opt})),
        "username"    => Ok(Box::new(UsernameSegment {options: opt})),
        "cwd"         => Ok(Box::new(CwdSegment {options: opt, global_config})),
        "git"         => Ok(Box::new(GitSegment {options: opt})),
        "env"         => Ok(Box::new(EnvSegment {options: opt})),
        "virtual_env" => Ok(Box::new(VirtualEnvSegment {options: opt})),
        "read_only"   => Ok(Box::new(ReadOnlySegment {options: opt})),
        "ssh"         => Ok(Box::new(SshSegment {options: opt})),
        "exit_code"   => Ok(Box::new(ExitCodeSegment {options: opt, prev_error: prev_error})),
        "stdout"      => Ok(Box::new(StdoutSegment {options: opt})),
        "jobs"        => Ok(Box::new(JobsSegment {options: opt})),
        "root"        => Ok(Box::new(RootSegment {options: opt , prev_error: prev_error})),                
        _             => Err(format!("Path segment not found: {}", s)),
    }
}


const DEFAULT_CONFIG:&str = r###"
{
    "segments": [
        "username",
        "hostname",
        "read_only",
        "exit_code",
        "cwd",
        "git",
        "root"
    ]
}
"###;

const WORKING_DIR_CF: &str = "prompter.json";
const HOME_DIR_CF: &str    = "/.prompter.json";
const STD_DIR_CF: &str     = "/prompter/config.json";
const ERR_MSG: &str = "Unable to read file";

fn fetch_config_as_string() -> String {
    if Path::new(WORKING_DIR_CF).exists() {
        return fs::read_to_string(WORKING_DIR_CF).expect(ERR_MSG);
    }

    let mut home_file = String::new();
    
    if let Ok(home_var) = env::var("HOME") {
        home_file.push_str(&home_var);
    } else {
        eprintln!("HOME variable not set. Something is very broken here.");
        return String::from(DEFAULT_CONFIG);
    }
        
    home_file.push_str(HOME_DIR_CF);

    if Path::new(&home_file).exists() {
        return fs::read_to_string(&home_file).expect(ERR_MSG);
    }

    let mut fallback = String::new();
    
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        fallback.push_str(&xdg);
        fallback.push_str(STD_DIR_CF);
    } else {
        fallback.push_str(&env::var("HOME").expect("HOME not set. Get a life."));
        fallback.push_str("/.config");
        fallback.push_str(STD_DIR_CF);
    }

    if Path::new(&fallback).exists() {
        return fs::read_to_string(&fallback).expect(ERR_MSG);
    } else {
        return String::from(DEFAULT_CONFIG);
    }
}

fn get_config() -> ConfigMap {
    let config_source = fetch_config_as_string();

    match serde_json::from_str(&config_source) {
        Ok(config) => return config,
        Err(err) => {
            eprintln!("Configfile broken: {}", err);
            return HashMap::new();
        }
    }
}

fn get_cwd_config(map: &ConfigMap, name: &str) -> Option<ConfigMap> {
    if let Some(cfg) = map.get(name) {
        let cwd_config: ConfigMap = serde_json::from_value(cfg.clone()).expect("Error parsing JSON.");
        return Some(cwd_config);
    }
    return None;
} 


fn main() {
    let mut prev_error: i32 = 0;
    
    // get return value
    // Kick this off with double-dashes:
    // cargo run -- --generate-config
    // cargo run -- 1
    
    if env::args().len() < 2 {
        eprintln!("Need a parameter");
        process::exit(1);
    } 
    
    for arg in env::args().skip(1) {
        if arg == "--generate-config" {
            println!("{}",DEFAULT_CONFIG);
            process::exit(0);
        } else if arg == "--v" {
            println!("{}", constants::VERSION);
            process::exit(0);
        } else {
            // it must be a integer ...
            if let Ok(val) = arg.parse::<i32>() {
                prev_error = val;
            } else {
                eprintln!("Not an integer");
                process::exit(1);
            }
        }
    }

    let map: ConfigMap = get_config();
    println!("{}", create_prompt(map, prev_error));
}

fn create_prompt(map: ConfigMap, prev_error: i32) -> String {
    let mut segment_chain: Vec<Box<dyn Segment>> = Vec::new();

    for (key, value) in &map {
        if key.to_lowercase() == "segments" {
            let segment_definition: Vec<Value> = serde_json::from_value(value.clone()).expect("Error parsing JSON.");
            
            for item in segment_definition {
                match item {
                    Value::String(s) => {

                        let mut global_config: ConfigMap = ConfigMap::new();
                        
                        if s == "cwd" {
                            if let Some(cwd_config) = get_cwd_config(&map, "cwd") {
                                // println!("cwd_config: {:?}", cwd_config);
                                global_config = cwd_config;
                            }
                        }
                        
                        match get_segment_for_string(s, None, prev_error, Some(global_config)) {
                            Ok(value) => segment_chain.push(value),
                            Err(reason) => eprintln!("Error: {}", reason),
                        }
                    },
                    Value::Object(o) => {
                        if o.contains_key("type") {
                            let seg = o["type"].as_str().expect("key named type not found in map.");

                            match get_segment_for_string(seg.to_string(), Some(o.clone()), prev_error, None) {
                                Ok(value) => segment_chain.push(value),
                                Err(reason) => eprintln!("Error: {}", reason),
                                }
                        }
                    },
                    _ => eprintln!("Not usable"),
                }
            }
        }
    }

    let prompt = Prompt {theme: default_theme(),
                         symbols: get_symbolset_for_name("patched")};
    
    // compute
    let result_set = compute_chain(segment_chain, &prompt);

    return render_prompt(result_set, &prompt);
}

fn compute_chain(segment_chain: Vec<Box<dyn Segment>>, prompt: &Prompt) -> Vec<ResultSet> {

    let mut result_set: Vec<ResultSet> = Vec::new();

    for seg in segment_chain.iter() {
        let compute_result = seg.compute(prompt);

        if ! compute_result.0.is_empty() { 
            result_set.push(compute_result);
        }
    }
    return result_set;
}


fn render_prompt(segment_chain: Vec<ResultSet>, prompt: &Prompt) -> String {
    let mut ps1: String = String::from("");

    let length = segment_chain.len();
    let mut idx = 0;

    for seg in segment_chain.iter() {
        let (segment_data, fg, bg, separator, separator_fg) = seg;
        
        ps1.push_str(&painter::fgcolor(*fg));
        ps1.push_str(&painter::bgcolor(*bg));        
        ps1.push_str(&segment_data);

        if !((idx + 1) == length) {
            let (_, _, next_bg, _, _) = segment_chain[idx + 1];
            ps1.push_str(&painter::bgcolor(next_bg));  
        } else {
            ps1.push_str(&painter::reset());
        }

        if *separator_fg != prompt.theme[RESET] {
            ps1.push_str(&painter::fgcolor(*separator_fg));
        } else {
            ps1.push_str(&painter::fgcolor(*bg));
        }
        
        if separator == "" {
            ps1.push_str(prompt.symbols.separator);
        } else {
            ps1.push_str(&separator);
        }
        

        idx += 1;
    }

    ps1.push_str(&painter::reset());
    ps1.push_str(" ");

    return ps1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_json_parsing() {
        // from json.org: "A string is a sequence of zero or more Unicode characters, wrapped in double quotes"
        let data = r##"{ "segments": [ "hostname" ] }"##;
        println!("Input: {}", data);
        let map: ConfigMap = serde_json::from_str(&data).expect("Error parsing JSON.");        

        assert_eq!(create_prompt(map, 0), r#"\[\e[38;5;250m\]\[\e[48;5;238m\] \h \[\e[0m\]\[\e[38;5;238m\]\[\e[0m\] "#)
        
    }
}    
