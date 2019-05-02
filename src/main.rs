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

    let fg = String::from("\x1B[38;2;");
    let bg = String::from("\x1B[48;2;");

    //let purple = "177;98;134m";
    //let orange = "214;93;14m";

    let path_part = PromptPart {
        fg: fg.clone() + "40;40;40m",
        bg: bg.clone() + "104;157;106m",
        inverse: fg.clone() + "104;157;106m",
        text: path_parser.path(2)
    };

    let jobs = PromptPart {
        bg: bg.clone() + "69;133;136m",
        fg: fg.clone() + "40;40;40m",
        inverse: fg.clone() + "69;133;136m",
        text: jobs::get_jobs()
    };
    let git = PromptPart {
        fg: fg.clone() + "104;157;106m",
        bg: bg.clone() + "40;40;40m",
        inverse: fg.clone() + "40;40;40m",
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
                    String::from("\x1B[38;2;0m")
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
