mod jobs;
mod git;
mod path;

struct PromptPart {
    fg: String,
    bg: String,
    inverse: String,
    text: Option<String>
}

fn main() {
    let path_parser = path::PathParser {
        parsers: vec![&path::PathParser::git, &path::PathParser::default],
    };

    let sym = String::from("\u{e0b0}");
    let esc = String::from("\x1B[");
    let fg = esc.clone() + "3";
    let bg = esc.clone() + "4";

    let black   = "0";
    let red     = "1";
    let green   = "2";
    let orange  = "3";
    let blue    = "4";
    let magenta = "5";
    let cyan    = "6";
    let white   = "7";

    let fg_black   = fg.clone() + black + "m";
    let fg_red     = fg.clone() + red + "m";
    let fg_green   = fg.clone() + green + "m";
    let fg_orange  = fg.clone() + orange + "m";
    let fg_blue    = fg.clone() + blue + "m";
    let fg_magenta = fg.clone() + magenta + "m";
    let fg_cyan    = fg.clone() + cyan + "m";
    let fg_white   = fg.clone() + white + "m";

    let bg_black=bg.clone() + black + "m";
    let bg_red=bg.clone() + red + "m";
    let bg_green=bg.clone() + green + "m";
    let bg_orange=bg.clone() + orange + "m";
    let bg_blue=bg.clone() + blue + "m";
    let bg_magenta=bg.clone() + magenta + "m";
    let bg_cyan=bg.clone() + cyan + "m";
    let bg_white=bg.clone() + white + "m";

    let path_part = PromptPart {
        fg: fg_black.clone(),
        bg: bg_cyan.clone(),
        inverse: fg_cyan.clone(),
        text: path_parser.path(2)
    };

    let jobs = PromptPart {
        fg: fg_black.clone(),
        bg: bg_blue.clone(),
        inverse: fg_blue.clone(),
        text: jobs::get_jobs()
    };
    let git = PromptPart {
        fg: fg_cyan.clone(),
        bg: bg_black.clone(),
        inverse: fg_black.clone(),
        text: git::get_branch()
    };

    let mut parts = vec![jobs, path_part, git];
    for i in 0..parts.len() {
        if parts[i].text.is_none() {
            parts.remove(i);
        }
    }

    for i in 0..parts.len() {
        let part = &parts[i];
        match &part.text {
            Some(text) => {
                let end_bg = if i < parts.len() - 1 {
                    parts[i + 1].bg.clone()
                }
                else {
                    String::from("\x1B[0m")
                };
                print!("%{{{}{}%}} {} %{{{}{}%}}\u{e0b0}",
                       part.fg, part.bg, text, end_bg, part.inverse);
            },
            None => {
                ()
            }
        }
    }
    print!("%{{\x1B[0m%}}\n:");
}
