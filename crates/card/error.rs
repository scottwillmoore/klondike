#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    Invalid,
    TooLong,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                ParseError::TooShort => "too few characters",
                ParseError::Invalid => "invalid characters",
                ParseError::TooLong => "too many characters",
            }
        )
    }
}

pub(crate) fn parse_char(str: &str) -> Result<char, ParseError> {
    let mut chars = str.chars();
    match (chars.next(), chars.next()) {
        (None, _) => Err(ParseError::TooShort),
        (Some(char), None) => Ok(char),
        _ => Err(ParseError::TooLong),
    }
}
