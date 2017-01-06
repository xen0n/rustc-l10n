use term::{self, StderrTerminal};

use super::errors::*;
use super::spec;

type Terminal = StderrTerminal;

pub trait Render {
    fn render(&self, &mut Box<Terminal>) -> Result<()>;
}


impl Render for spec::Diagnostic {
    fn render(&self, t: &mut Box<Terminal>) -> Result<()> {
        self.level.set_attr(false, t)?;
        self.level.output(t)?;
        if let Some(ref code) = self.code {
            self.level.set_attr(false, t)?;
            write!(t, "[{}]", code.code)?;
        }
        t.attr(term::Attr::Bold)?;
        t.fg(term::color::WHITE)?;
        writeln!(t, ": {}", self.message)?;
        t.reset()?;

        if !self.spans.is_empty() {
            render_spans(self, t)?;
        }

        writeln!(t, "")?;

        Ok(())
    }
}

impl spec::ErrorLevel {
    fn set_attr(&self, in_span: bool, t: &mut Box<Terminal>) -> Result<()> {
        t.attr(term::Attr::Bold)?;
        match *self {
            spec::ErrorLevel::Ice | spec::ErrorLevel::Error => {
                t.fg(term::color::RED)?;
            }
            spec::ErrorLevel::Warning => t.fg(term::color::YELLOW)?,
            spec::ErrorLevel::Note => {
                t.fg(if in_span {
                        term::color::WHITE
                    } else {
                        term::color::GREEN
                    })?;
            }
            spec::ErrorLevel::Help => {
                t.fg(term::color::WHITE)?;
            }
        }

        Ok(())
    }

    fn output(&self, t: &mut Box<Terminal>) -> Result<()> {
        write!(t, "{}", match *self {
            spec::ErrorLevel::Ice => "error: internal compiler error",
            spec::ErrorLevel::Error => "error",
            spec::ErrorLevel::Warning => "warning",
            spec::ErrorLevel::Note => "note",
            spec::ErrorLevel::Help => "help",
        })?;
        t.reset()?;

        Ok(())
    }
}

fn render_spans(diag: &spec::Diagnostic, t: &mut Box<Terminal>) -> Result<()> {
    let sps = &diag.spans;

    // sort the spans by starting line
    let sps = {
        let mut sps: Vec<_> = sps.iter().collect();
        sps.sort_by(|a, b| a.line_start.cmp(&b.line_start));
        sps
    };

    // get maximum line number for left-padding the line numbers
    let max_lineno = sps[sps.len() - 1].line_start;
    let lineno_width = format!("{}", max_lineno).len();  // very crude but enough

    {
        // find primary span
        let primary_sp = sps.iter().filter(|sp| sp.is_primary).take(1).next();

        // print the leading line of spans
        // '...  --> {filename}:{start line}:{start column}'
        // '...   |'  -- arrow is aligned with the vertical column below
        if let Some(sp) = primary_sp {
            let lead_text = format!("{}:{}:{}", sp.file_name, sp.line_start, sp.column_start);
            render_lineno(LineNumberPrefix::Leading(lead_text), lineno_width, t)?;
        }
    }

    // TODO: match rustc's behavior here!
    render_lineno(LineNumberPrefix::Empty, lineno_width, t)?;
    writeln!(t, "")?;
    for sp in sps {
        let mut lineno = sp.line_start;
        for line in &sp.text {
            render_lineno(LineNumberPrefix::Lineno(lineno), lineno_width, t)?;
            writeln!(t, "{}", line.text)?;
            lineno += 1;
        }
        render_lineno(LineNumberPrefix::Empty, lineno_width, t)?;
        render_highlight(sp.column_start, sp.column_end, sp.is_primary, &sp.label, t)?;
    }

    // children are notes or helps in current rustc
    for child_diag in &diag.children {
        match child_diag.level {
            spec::ErrorLevel::Note | spec::ErrorLevel::Help => {
                // print notes along with spans
                render_lineno(LineNumberPrefix::Note, lineno_width, t)?;
                child_diag.level.set_attr(true, t)?;
                child_diag.level.output(t)?;
                writeln!(t, ": {}", child_diag.message)?;
            }
            _ => unimplemented!(),
        }
    }

    Ok(())
}


enum LineNumberPrefix {
    Leading(String),
    Lineno(usize),
    Empty,
    Note,
}


fn render_lineno(x: LineNumberPrefix, width: usize, t: &mut Box<Terminal>) -> Result<()> {
    t.attr(term::Attr::Bold)?;
    t.fg(term::color::BLUE)?;

    match x {
        LineNumberPrefix::Lineno(x) => {
            // TODO: avoid allocation here
            let lineno_str = format!("{}", x);
            let pad_width = width - lineno_str.len();
            for _ in 0..pad_width {
                write!(t, " ")?;
            }
            write!(t, "{} | ", lineno_str)?;
        }
        LineNumberPrefix::Leading(s) => {
            for _ in 0..width {
                write!(t, " ")?;
            }
            write!(t, "--> ")?;
            t.reset()?;
            writeln!(t, "{}", s)?;
        }
        LineNumberPrefix::Empty | LineNumberPrefix::Note => {
            let sep = match x {
                LineNumberPrefix::Empty => "|",
                LineNumberPrefix::Note => "= ",
                _ => unreachable!(),
            };
            for _ in 0..width {
                write!(t, " ")?;
            }
            write!(t, " {}", sep)?;
        }
    }

    t.reset()?;

    Ok(())
}

fn render_highlight<S: AsRef<str>>(col_start: usize,
                                   col_end: usize,
                                   is_primary: bool,
                                   text: &Option<S>,
                                   t: &mut Box<Terminal>)
                                   -> Result<()> {
    // TODO: multi-line spans
    let ch = if is_primary { '^' } else { '-' };
    for _ in 0..col_start {
        write!(t, " ")?;
    }
    for _ in col_start..col_end {
        write!(t, "{}", ch)?;
    }
    if let Some(ref text) = *text {
        writeln!(t, " {}", text.as_ref())?;
    }

    Ok(())
}
