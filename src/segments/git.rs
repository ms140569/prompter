// varius ways to disable unused code warnings are here:
// https://stackoverflow.com/a/25877389
#![allow(dead_code)]

use serde_json::{Value, Map};
use Segment;
use ResultSet;
use std::process::Command;
use std::str;
use themes::*;
use regex::Regex;
use prompt::Prompt;

pub struct RepoSet {
    pub symbol: &'static str,
    pub fg: usize,
    pub bg: usize,
}

const DETACHED:RepoSet   = RepoSet{symbol: "\u{2693}",         fg: RESET, bg: RESET};

const AHEAD:RepoSet      = RepoSet{symbol: "\u{2B06}",         fg: GIT_AHEAD_FG, bg: GIT_AHEAD_BG};
const BEHIND:RepoSet     = RepoSet{symbol: "\u{2B07}",         fg: GIT_BEHIND_FG, bg: GIT_BEHIND_BG};
const STAGED:RepoSet     = RepoSet{symbol: "\u{2714}",         fg: GIT_STAGED_FG, bg: GIT_STAGED_BG};
const CHANGED:RepoSet    = RepoSet{symbol: "\u{270E}",         fg: GIT_NOTSTAGED_FG, bg: GIT_NOTSTAGED_BG};
const NEW:RepoSet        = RepoSet{symbol: "?",                fg: GIT_UNTRACKED_FG, bg: GIT_UNTRACKED_BG};
const CONFLICTED:RepoSet = RepoSet{symbol: "\u{273C}",         fg: GIT_CONFLICTED_FG, bg: GIT_CONFLICTED_BG};

const STASH:RepoSet      = RepoSet{symbol: "\u{2398}",         fg: RESET, bg: RESET};
const GIT:RepoSet        = RepoSet{symbol: "\u{E0A0}",         fg: RESET, bg: RESET};
const HG:RepoSet         = RepoSet{symbol: "\u{263F}",         fg: RESET, bg: RESET};
const BZR:RepoSet        = RepoSet{symbol: "\u{2B61}\u{20DF}", fg: RESET, bg: RESET};
const FOSSIL:RepoSet     = RepoSet{symbol: "\u{2332}",         fg: RESET, bg: RESET};
const SVN:RepoSet        = RepoSet{symbol: "\u{2446}",         fg: RESET, bg: RESET};

fn get_fg_bg_for_repo_set(theme: Theme, reposet: &RepoSet) -> (Colorcode, Colorcode) {
    return (theme[reposet.fg], theme[reposet.bg]);
}

#[derive(Debug)]
pub struct GitSegment {
    pub options: Option<Map<String, Value>>
}

pub struct BranchInfo {
    pub local: String,
    pub remote: String,
    pub ahead: i32,
    pub behind: i32,
}

fn parse_git_branch_info(input: &str) -> Option<BranchInfo> {
    let re = Regex::new(
        r###"^## (?P<local>\S+?)(\.{3}(?P<remote>\S+?)( \[(ahead (?P<ahead>\d+)(, )?)?(behind (?P<behind>\d+))?\])?)?$"###
    ).expect("Problem creating regular expressen. Bail out.");
    
    let caps = match re.captures(input) {
        None => return None,
        Some(val) => val,
    };

    let mut local = String::new();
    let mut remote = String::new();
    let mut ahead = 0;
    let mut behind = 0;
    
    if let Some(m) = caps.name("local") {
        local = m.as_str().to_string();
    }

    if let Some(m) = caps.name("remote") {
        remote = m.as_str().to_string();
    }

    if let Some(m) = caps.name("ahead") {
        if let Ok(v) = m.as_str().parse::<i32>() {
            ahead = v;
        }
    }

    if let Some(m) = caps.name("behind") {
        if let Ok(v) = m.as_str().parse::<i32>() {
            behind = v;
        }
    }

    Some(BranchInfo {local, remote, ahead, behind})
}


fn number_or_blank(number: i32) -> String {
    if number < 2 {
        return String::new();
    } else {
        return number.to_string();
    }
}

pub struct RepoStats {
    pub new: i32,
    pub conflicted: i32,
    pub changed: i32,
    pub staged: i32,
    
    pub ahead: i32, // these two come from the sibling struct.
    pub behind: i32,
}

impl RepoStats {
    fn dirty(&self) -> bool {
        ( self.new + self.conflicted + self.changed + self.staged ) > 0
    }

    fn print(&self) {
        println!("New        : {}", self.new);
        println!("Conflicted : {}", self.conflicted);
        println!("Changed    : {}", self.changed);
        println!("Staged     : {}", self.staged);
        println!("Ahead      : {}", self.ahead);
        println!("Behind     : {}", self.behind);
    } 
}

fn parse_git_stats(lines: &[&str]) -> RepoStats {
    let mut new = 0;
    let mut conflicted = 0;
    let mut changed = 0;
    let mut staged = 0;
    
    for line in lines {
        let code = &line[..2];

        if code == "??" {
            new += 1;
        } else {
            match code {
                "DD" | "AU" | "UD" | "UA" | "DU" | "AA" | "UU" => conflicted += 1,
                _ => {
                    if code.chars().nth(1).unwrap_or(' ') != ' ' {
                        changed += 1;
                    }
                    if code.chars().nth(0).unwrap_or(' ') != ' '{
                        staged += 1;
                    }
                }
            }
        }
    }
    RepoStats{new, conflicted, changed, staged, ahead: 0, behind: 0}
}


fn get_git_detached_branch() -> String {
    let git_cmd_result = Command::new("git")
        .env("LANG", "C")
        .arg("describe")
        .arg("--tags")
        .arg("--always")
        .output();

    match git_cmd_result {
        Err(err) => {
            eprintln!("Failed to execute git: {}", err);
            return String::from("<git-err>");
        },
        Ok(output) => {
            if output.status.success() {
                return String::from_utf8(output.stdout).unwrap_or("*err*".to_string()).trim_end().to_string();
            } else {
                return String::from("Big Bang");
            }
        }
    }
}

fn add_vcs_details(theme: Theme, symbols: Symbols, upstream: Colorcode, repo_stats: RepoStats) -> (String, Colorcode) {
    let mut result = String::new();

    //          **************            
    // before > branch > a > b > follow

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.ahead, &AHEAD, upstream);
    result.push_str(&data);

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.behind, &BEHIND, cc);
    result.push_str(&data);

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.staged, &STAGED, cc);
    result.push_str(&data);

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.changed, &CHANGED, cc);
    result.push_str(&data);

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.new, &NEW, cc);
    result.push_str(&data);

    let ( data, cc ) = add_vcs_part(theme, symbols, repo_stats.conflicted, &CONFLICTED, cc);
    result.push_str(&data);

    (result, cc)
}

// if upstream != RESET than paint a separator with this color *before*
fn add_vcs_part(theme: Theme, symbols: Symbols, value: i32, repo_set: &RepoSet, upstream: Colorcode) -> (String, Colorcode) {
    if value < 1 { return (String::new(), upstream); }
    let (fg, bg) = get_fg_bg_for_repo_set(theme, repo_set);
    // separator
    let mut ret_val = String::from(format!("{}{}{}", painter::fgcolor(upstream), painter::bgcolor(bg), symbols.separator));
    // data
    ret_val.push_str(&format!("{}{} {}{} ", painter::fgcolor(fg), painter::bgcolor(bg), number_or_blank(value), repo_set.symbol));
    (ret_val, bg)
            
}


impl Segment for GitSegment {
    fn compute(&self, prompt: &Prompt) -> ResultSet { 

        // git status --porcelain -b
       
        let git_cmd_result = Command::new("git")
            .env("LANG", "C")
            .arg("status")
            .arg("--porcelain")
            .arg("-b")
            .output();

       
        if let Err(err) = git_cmd_result {
            eprintln!("Failed to execute git: {}", err);
            return (String::from("<git-err>"), prompt.theme[REPO_DIRTY_FG], prompt.theme[REPO_DIRTY_BG], String::new(), prompt.theme[RESET]);
        }

        let output = git_cmd_result.unwrap();
        let std_out_value = str::from_utf8(&output.stdout).unwrap();

        let lines:Vec<&str> = std_out_value.lines().collect();

        // no git repo, bail-out
        if lines.len() == 0 {
            return (String::new(), prompt.theme[RESET], prompt.theme[RESET], String::new(), prompt.theme[RESET]);
        }
        
        let branch_name;
        let branch_info;
        
        if let Some(bi) = parse_git_branch_info(lines[0]) {
            branch_name = bi.local.clone();
            branch_info = bi;
        } else {
            branch_name = get_git_detached_branch();
            branch_info = BranchInfo {local: String::new(), remote: String::new(), ahead: 0, behind: 0};
        }

        let mut repo_stats = parse_git_stats(&lines[1..]);

        repo_stats.ahead = branch_info.ahead;
        repo_stats.behind = branch_info.behind;

        let fg;
        let bg;
        
        if repo_stats.dirty() {
            fg = prompt.theme[REPO_DIRTY_FG];
            bg = prompt.theme[REPO_DIRTY_BG];        

        } else {
            fg = prompt.theme[REPO_CLEAN_FG];
            bg = prompt.theme[REPO_CLEAN_BG];        
        }

        let mut git_line: String = format!("{} ", branch_name);

        // repo_stats.print();

        let (data, cc) = add_vcs_details(prompt.theme, prompt.symbols, bg, repo_stats);
        
        git_line.push_str(&data);
        
        return (format!(" {}", git_line), fg, bg, String::new(), cc);


    }


}

/* Python seach strings is in fact a (silent) concatenation of two strings, see single quote two times between #1 an #2 capture group.
re.search('^## (?P<local>\S+?)''(\.{3}(?P<remote>\S+?)( \[(ahead (?P<ahead>\d+)(, )?)?(behind (?P<behind>\d+))?\])?)?$', text).groupdict()

"## master...origin/master"           -> {'behind': None, 'local': 'master', 'remote': 'origin/master', 'ahead': None}
"## master...origin/master [ahead 1]" -> {'behind': None, 'local': 'master', 'remote': 'origin/master', 'ahead': '1'}
*/

#[cfg(test)]
mod test {
    use super::*;

    // cargo test segments::git::test::test_regex_differences -- --nocapture
    // see: https://crates.io/crates/regex
    
    #[test]
    fn test_regex_differences() {

        // let input = "## master...origin/master [ahead 1]";
        let input = "## master...origin/master [ahead 1, behind 7]";

        println!("Processing: |{}|", input);
        
        let re = Regex::new(
            r###"^## (?P<local>\S+?)(\.{3}(?P<remote>\S+?)( \[(ahead (?P<ahead>\d+)(, )?)?(behind (?P<behind>\d+))?\])?)?$"###
            ).expect("Regex could not be parsed.");

        println!("Found : {}", re.is_match(input));

        let caps = re.captures(input).unwrap();

        println!("Len: {}", caps.len());
        
        println!("local  : {}", caps.name("local").unwrap().as_str());
        println!("remote : {}", caps.name("remote").unwrap().as_str());
        println!("ahead  : {}", caps.name("ahead").unwrap().as_str());
        println!("behind : {}", caps.name("behind").unwrap().as_str());
        
    }

}    

