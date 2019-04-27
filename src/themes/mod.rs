pub mod default;
pub mod crazy;
pub mod painter;

pub fn _get_color_templates(s: &str) -> String {
    match s {
        "bash" => String::from("\x1B[{}]"),
        "tcsh" => String::from("%%{\x1B%s%%}"),
        "zsh"  => String::from("%%{%%}"),
        "bare" => String::from(""),
        _      => String::from(""),                
    }
}

#[derive(Clone, Copy)]
pub struct Symbols {
    pub lock: &'static str,
    pub network: &'static str,
    pub separator: &'static str,
    pub separator_thin: &'static str,
}


pub fn get_symbolset_for_name(name: &str) -> Symbols {

    let patched = Symbols {lock: "\u{E0A2}", network: "SSH", separator: "\u{E0B0}", separator_thin: "\u{E0B1}" };

    match name {
        "compatible" => Symbols {lock: "RO", network: "SSH", separator: "\u{25B6}", separator_thin: "\u{276F}"},
        "patched"    => patched,
        "flat"       => Symbols {lock: "\u{E0A2}", network: "SSH", separator: "", separator_thin: ""},        
        _ =>         patched,
    }
}


pub const THEME_SIZE:usize = 53;

pub type Colorcode = i32;
pub type Theme = [Colorcode; THEME_SIZE];

pub const     RESET:usize = 0;
pub const     USERNAME_FG:usize = 1;
pub const     USERNAME_BG:usize = 2;
pub const     USERNAME_ROOT_BG:usize = 3;
pub const     HOSTNAME_FG:usize = 4;
pub const     HOSTNAME_BG:usize = 5;
pub const     HOME_SPECIAL_DISPLAY:usize = 6; // Value 0 -> False, 1 -> True
pub const     HOME_BG:usize = 7;
pub const     HOME_FG:usize = 8;
pub const     PATH_BG:usize = 9;
pub const     PATH_FG:usize = 10;
pub const     CWD_FG:usize = 11;
pub const     SEPARATOR_FG:usize = 12;
pub const     READONLY_BG:usize = 13;
pub const     READONLY_FG:usize = 14;
pub const     SSH_BG:usize = 15;
pub const     SSH_FG:usize = 16;
pub const     REPO_CLEAN_BG:usize = 17;
pub const     REPO_CLEAN_FG:usize = 18;
pub const     REPO_DIRTY_BG:usize = 19;
pub const     REPO_DIRTY_FG:usize = 20;
pub const     JOBS_FG:usize = 21;
pub const     JOBS_BG:usize = 22;
pub const     CMD_PASSED_BG:usize = 23;
pub const     CMD_PASSED_FG:usize = 24;
pub const     CMD_FAILED_BG:usize = 25;
pub const     CMD_FAILED_FG:usize = 26;
pub const     SVN_CHANGES_BG:usize = 27;
pub const     SVN_CHANGES_FG:usize = 28;
pub const     GIT_AHEAD_BG:usize = 29;
pub const     GIT_AHEAD_FG:usize = 30;
pub const     GIT_BEHIND_BG:usize = 31;
pub const     GIT_BEHIND_FG:usize = 32;
pub const     GIT_STAGED_BG:usize = 33;
pub const     GIT_STAGED_FG:usize = 34;
pub const     GIT_NOTSTAGED_BG:usize = 35;
pub const     GIT_NOTSTAGED_FG:usize = 36;
pub const     GIT_UNTRACKED_BG:usize = 37;
pub const     GIT_UNTRACKED_FG:usize = 38;
pub const     GIT_CONFLICTED_BG:usize = 39;
pub const     GIT_CONFLICTED_FG:usize = 40;
pub const     GIT_STASH_BG:usize = 41;
pub const     GIT_STASH_FG:usize = 42;
pub const     VIRTUAL_ENV_BG:usize = 43;
pub const     VIRTUAL_ENV_FG:usize = 44;
pub const     BATTERY_NORMAL_BG:usize = 45;
pub const     BATTERY_NORMAL_FG:usize = 46;
pub const     BATTERY_LOW_BG:usize = 47;
pub const     BATTERY_LOW_FG:usize = 48;
pub const     AWS_PROFILE_FG:usize = 49;
pub const     AWS_PROFILE_BG:usize = 50;
pub const     TIME_FG:usize = 51;
pub const     TIME_BG:usize = 52;


#[cfg(test)]
mod test {
    use super::*;
    use themes::default::default_theme;
    
    #[test]
    fn test_color_output() {
        println!("\x1B[38;5;250m\x1B[48;5;240m \\u \x1B[48;5;238m\x1B[38;5;240m\x1B[38;5;250m\x1B[48;5;238m \\h \x1B[48;5;237m\x1B[38;5;238m\x1B[0m ");
    }

    #[test]
    fn test_color_output_short() {
        println!("\x1B[38;5;250m\x1B[48;5;240m SHORT \x1B[48;5;238m\x1B[38;5;240m\x1B[0m ");
        // println!("\x1B[38;5;250m\x1B[48;5;240m SHORT2 \x1B[0m ");
        // println!("\x1B[38;5;250m SHORT3 \x1B[0m ");
        // println!("\x1B[38;5;250m SHORT4 ");                        
    }


    #[test]
    fn test_painter() {
        println!("\nTEST: --|{}{}VALUE{}{}{}|--",
                 painter::fgcolor(250),
                 painter::bgcolor(240),
                 painter::bgcolor(238),
                 painter::fgcolor(240),
                 painter::reset()

        );

        let theme = default_theme();
        
        assert!(theme[USERNAME_FG] == 250); // see default theme!
    }

    #[test]
    fn test_symbols_lookup() {
        let symbols = get_symbolset_for_name("flat");
        assert!(symbols.network == "SSH");
        assert!(symbols.lock == "\u{E0A2}");        
    }

}    

