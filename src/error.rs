#[derive(Debug)]
pub enum ErrorKind {
    SyntaxError,
    TypeError,
}

pub struct Error {
    pub(crate) line: usize,
    pub(crate) content: String,
    pub(crate) msg: String,
    pub(crate) kind: ErrorKind,
}

impl Error {
    pub fn new(line: usize, content: String, msg: String, kind: ErrorKind) -> Self {
        Self {
            line,
            content,
            msg,
            kind,
        }
    }

    pub fn message(&self, interpreted: bool) -> String {
        if interpreted {
            format!("{:?}: {}\n{}\n", self.kind, self.msg, self.content)
        } else {
            format!(
                "{:?}: {}\n{} | {}\n",
                self.kind, self.msg, self.line, self.content
            )
        }
    }

    pub fn report(&self, interpreted: bool) {
        eprintln!("{}", self.message(interpreted));
    }

    pub fn contextualize(&mut self, src: &str) {
        if let Some(line_content) = src.lines().nth(self.line - 1) {
            self.content = line_content.to_string();
        }
    }
}
