use std::fmt;
use std::fmt::Formatter;
use colored::Colorize;

#[derive(Debug)]
pub enum Message {
    SOURCE1,
    SOURCE2,
    JSON1,
    JSON2,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Message::SOURCE1 => "Could not read source1.",
            Message::SOURCE2 => "Could not read source2.",
            Message::JSON1 => "Could not parse source1.",
            Message::JSON2 => "Could not parse source2.",
        };

        write!(f, "{}", message.red())
    }
}