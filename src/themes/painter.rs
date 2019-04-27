use themes::Colorcode;

/* 
color: \[\e[38;5;250m\]

reset: \[\e[0m\]
*/

pub fn color(prefix: &str, code: Colorcode) -> String {
    return String::from(format!("\\[\\e[{};5;{}m\\]", prefix, code));
}

pub fn fgcolor(code: Colorcode) -> String {
    color("38", code)
}

pub fn bgcolor(code: Colorcode) -> String {
    color("48", code)
}

pub fn reset() -> String {
    return String::from("\\[\\e[0m\\]");
}
