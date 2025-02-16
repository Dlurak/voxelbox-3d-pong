use ansi_term::Color;

#[allow(dead_code)]
pub enum Severity {
    Critical,
    Warning,
    Log,
    Success,
}

impl Severity {
    pub fn prefix(&self) -> String {
        match self {
            Self::Success => Color::Green.bold().paint("[Success]"),
            Self::Log => Color::Blue.bold().paint("[Log]"),
            Self::Warning => Color::Yellow.bold().paint("[Warn]"),
            Self::Critical => Color::Red.bold().paint("[Critical]"),
        }
        .to_string()
    }
}

#[macro_export]
macro_rules! log {
    ($severity:ident, $($arg:tt)*) => {{
        print!("{} {}\n", $crate::Severity::$severity.prefix(), format!($($arg)*));
    }};
}
