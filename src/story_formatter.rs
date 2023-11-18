use std::io::Write;
use terminal_size::{terminal_size, Width};

fn name_ok_fail(flag: bool) -> &'static str {
    if flag {
        "ok"
    } else {
        "FAIL"
    }
}

fn name_y_n(flag: bool) -> &'static str {
    if flag {
        "Y"
    } else {
        "N"
    }
}

fn print_wrapped(prefix: &str, border: &str, text: &str) {
    let terminal_width = if let Some((Width(w), _)) = terminal_size() {
        // make terminal size to be reasonably wide
        w.max(20) as usize
    } else {
        80
    };
    let width = border.chars().count().max(prefix.chars().count());
    let text: Vec<char> = text.chars().collect();
    let text_width = terminal_width - width;
    let mut cursor = 0;
    let mut tokens = vec![];
    let next_range = |c: usize| c..(c + text_width).min(text.len());
    let first_line: String = text[next_range(cursor)].iter().collect();
    tokens.push(format!("{prefix:<width$}{first_line}"));
    cursor += text_width;
    while cursor < text.len() {
        let next_line: String = text[next_range(cursor)].iter().collect();
        tokens.push(format!("{border:<width$}{next_line}"));
        cursor += text_width;
    }
    if let Some((last, tokens)) = tokens.split_last() {
        for t in tokens {
            println!("{t}");
        }
        print!("{last}");
    }
}

#[derive(Clone)]
pub(crate) struct StoryFormatter {
    section_stack: Vec<String>,
    next_is_separator: bool,
}

impl StoryFormatter {
    pub fn new() -> Self {
        Self {
            section_stack: vec![],
            next_is_separator: false,
        }
    }

    pub fn push<Name>(&mut self, section_name: Name)
    where
        Name: Into<String>,
    {
        self.section_stack.push(section_name.into());
    }

    pub fn section<Name, F>(&mut self, section_name: Name, section_fn: F) -> Result<(), ()>
    where
        Name: Into<String>,
        F: FnOnce(&mut Self) -> Result<(), ()>,
    {
        self.push(section_name);
        let result = section_fn(self);
        self.section_result(result.is_ok());
        result
    }

    fn put_separator(&mut self) {
        if self.next_is_separator {
            self.next_is_separator = false;
            println!();
        }
    }

    fn section_name(&self) -> String {
        self.section_stack
            .iter()
            .map(|s| {
                let square_brackets = s.contains('.');
                let mut tokens = vec![];
                if square_brackets {
                    tokens.push("[".to_owned());
                }
                tokens.push(s.clone());
                if square_brackets {
                    tokens.push("]".to_owned());
                }
                tokens.join("")
            })
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn checklist<Name, F>(&mut self, title: Name, checklist_fn: F) -> Result<(), ()>
    where
        Name: Into<String>,
        F: FnOnce(&mut Self) -> Result<(), ()>,
    {
        // reset `separator` flag, not needed before checklist
        self.next_is_separator = false;
        self.push(title);
        self.checklist_title();
        let result = checklist_fn(self);
        self.checklist_result(result.is_ok());
        result
    }

    pub fn checklist_title(&self) {
        println!(" _");

        print_wrapped("|", "|", &self.section_name());
        println!();

        println!("|");
    }

    // todo: use box-drawing characters?
    // todo: add colors (if terminal supports)?
    pub fn checklist_item(&self, ok: bool, i: usize, title: &str) {
        let prefix = format!("|-[{:^3}] ", name_y_n(ok));
        let title = format!("{i}.{title}");
        print_wrapped(&prefix, "|", &title);
        println!();
    }

    pub fn checklist_result(&mut self, ok: bool) {
        println!("|");
        println!("|> {}", name_ok_fail(ok));
        self.section_stack.pop();
        self.next_is_separator = true;
    }

    pub fn checklist_title_note(&self, note: &str) {
        let note = format!("*{note}*");
        print_wrapped("| ", "|", &note);
        println!();
        println!("|");
    }

    pub fn checklist_note(&self, note: &str) {
        println!("|");
        let note = format!("*{note}*");
        print_wrapped("| ", "|", &note);
        println!();
    }

    pub fn playbook_header(&mut self, header: &str) {
        println!("Applying playbook: {}", header);
        self.next_is_separator = true;
    }

    pub fn playbook_result(header: &str, ok: bool) {
        println!();
        print!("{}", header);
        println!();
        println!();
        print!(" {}", name_ok_fail(ok).to_uppercase());
        println!();
        println!();
    }

    pub fn section_result(&mut self, ok: bool) {
        self.put_separator();
        println!("{}|> {}", self.section_name(), name_ok_fail(ok));
        self.section_stack.pop();
    }

    pub fn process<Name, F>(&mut self, title: Name, process_fn: F) -> Result<(), ()>
    where
        Name: Into<String>,
        F: FnOnce(&mut Self) -> Result<(), ()>,
    {
        self.push(title);
        self.section_process();
        print!("...applying");
        // trying to flush, but not hard, ignoring if error
        let _ = std::io::stdout().flush();
        let result = process_fn(self);
        let result_name = if result.is_ok() {
            "...done!"
        } else {
            "...FAIL!"
        };
        println!("{result_name}");
        self.section_stack.pop();
        result
    }

    pub fn section_process(&mut self) {
        self.put_separator();
        print!("{}", self.section_name());
        print!("|> ");
    }
}
