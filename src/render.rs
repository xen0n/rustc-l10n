use std::fmt;

use super::spec;


pub trait Render {
    fn render(&self);
}


impl Render for spec::Diagnostic {
    fn render(&self) {
        print!("{}", self.level);
        if let Some(ref code) = self.code {
            print!("[{}]", code.code);
        }
        print!(": ");
        println!("{}", self.message);

        if !self.spans.is_empty() {
            render_spans(self);
        }

        println!();
    }
}

impl fmt::Display for spec::ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            spec::ErrorLevel::Ice => "error: internal compiler error",
            spec::ErrorLevel::Error => "error",
            spec::ErrorLevel::Warning => "warning",
            spec::ErrorLevel::Note => "note",
            spec::ErrorLevel::Help => "help",
        })
    }
}

fn render_spans(diag: &spec::Diagnostic) {
    let sps = &diag.spans;

    // find primary span
    let primary_sp = sps.iter().filter(|sp| sp.is_primary).take(1).next();
    if let Some(sp) = primary_sp {
        // print the leading line of spans
        // ' --> {filename}:{start line}:{start column}
        print!(" --> ");
        println!("{}:{}:{}", sp.file_name, sp.line_start, sp.column_start);
    }

    // sort the spans by starting line
    let sps = {
        let mut sps: Vec<_> = sps.iter().collect();
        sps.sort_by(|a, b| a.line_start.cmp(&b.line_start));
        sps
    };

    // get maximum line number for left-padding the line numbers
    let max_lineno = sps[sps.len() - 1].line_start;
    let lineno_width = format!("{}", max_lineno).len();  // very crude but enough

    // TODO: match rustc's behavior here!
    render_lineno(LineNumberPrefix::Empty, lineno_width);
    println!();
    for sp in sps {
        let mut lineno = sp.line_start;
        for line in &sp.text {
            render_lineno(LineNumberPrefix::Lineno(lineno), lineno_width);
            println!("{}", line.text);
            lineno += 1;
        }
        render_lineno(LineNumberPrefix::Empty, lineno_width);
        render_highlight(sp.column_start, sp.column_end, sp.is_primary, &sp.label);
    }

    // children are notes or helps in current rustc
    for child_diag in &diag.children {
        match child_diag.level {
            spec::ErrorLevel::Note | spec::ErrorLevel::Help => {
                // print notes along with spans
                render_lineno(LineNumberPrefix::Note, lineno_width);
                print!("{}", child_diag.level);
                println!(": {}", child_diag.message);
            }
            _ => unimplemented!(),
        }
    }
}


enum LineNumberPrefix {
    Lineno(usize),
    Empty,
    Note,
}


fn render_lineno(x: LineNumberPrefix, width: usize) {
    match x {
        LineNumberPrefix::Lineno(x) => {
            // TODO: avoid allocation here
            let lineno_str = format!("{}", x);
            let pad_width = width - lineno_str.len();
            for _ in 0..pad_width {
                print!(" ");
            }
            print!("{} | ", lineno_str);
        }
        LineNumberPrefix::Empty | LineNumberPrefix::Note => {
            let sep = match x {
                LineNumberPrefix::Empty => "|",
                LineNumberPrefix::Note => "= ",
                _ => unreachable!(),
            };
            for _ in 0..width {
                print!(" ");
            }
            print!(" {}", sep);
        }
    }
}

fn render_highlight<S: AsRef<str>>(col_start: usize,
                                   col_end: usize,
                                   is_primary: bool,
                                   text: &Option<S>) {
    // TODO: multi-line spans
    let ch = if is_primary { '^' } else { '-' };
    for _ in 0..col_start {
        print!(" ");
    }
    for _ in col_start..col_end {
        print!("{}", ch);
    }
    if let Some(ref text) = *text {
        println!(" {}", text.as_ref());
    }
}
