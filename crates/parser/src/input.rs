use std::convert::TryFrom;
use text_size::{TextRange, TextSize, TextSized};

#[derive(Debug, Clone)]
pub struct Input {
    str: Box<str>,
    range: TextRange,
}

impl Input {
    pub fn chomp(&mut self, len: usize) -> TextRange {

        let range = match self
            .as_ref()
            .char_indices()
            .nth(len - 1)
            .and_then(|(last, c)| TextSize::try_from(last + c.len_utf8()).ok())
        {
            Some(last) => TextRange(self.range.start(), self.range.start() + last),
            None => self.range
        };
        self.set_cursor(range.end());

        range
    }

    pub fn cursor(&self) -> TextSize {
        self.range.start()
    }

    pub fn set_cursor(&mut self, cursor: TextSize) {
        self.range = TextRange(cursor, self.range.end());
    }

    pub fn set_range(&mut self, range: TextRange) {
        self.range = range;
    }

    pub fn range_span(&self, range: TextRange) -> &str {
        &self.str[range]
    }
}

impl From<&'_ str> for Input {
    fn from(input: &str) -> Self {
        let str: Box<str> = Box::from(input);
        Self {
            str,
            range: TextRange(TextSize::zero(), input.text_size()),
        }
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        &self.str[self.range]
    }
}