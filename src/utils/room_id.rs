use displaydoc::Display;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// The room identifier.
///
/// The room ID is a 6 upper cased character long string.
/// Example: `ABCDEF`, `BBBBBB`, `AAAAAA`
///
/// To convert a string to a room ID, use the `FromStr` trait.
/// To convert a room ID to a string, use the `Display` trait.
///
/// ```
/// # use utils::room_id::RoomID;
/// let room_id: RoomID = "ABCDEF".parse().unwrap();
/// let room_str = room_id.to_string();
///
/// assert_eq!(room_str, "ABCDEF");
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoomID {
    value: u32,
}

impl RoomID {
    /// Create a new room ID from a number.
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Get the value of the room ID.
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomParseError {
    /// The room ID must be 6 characters long.
    InvalidLength,
    /// The room ID must only contain uppercase letters.
    InvalidChar,
}

impl RoomID {
    const CHAR_LENGTH: usize = 6;
}

impl FromStr for RoomID {
    type Err = RoomParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RoomParseError::*;
        if s.len() != Self::CHAR_LENGTH {
            return Err(InvalidLength);
        }

        let mut id = 0;
        for (i, c) in s.chars().rev().enumerate() {
            // check if the character is between A and Z
            if c.is_ascii_uppercase() {
                // convert the character to a number
                let num = c as u32 - 'A' as u32;
                // add the number to the id
                id += num * 26u32.pow(i as u32);
            } else {
                return Err(InvalidChar);
            }
        }
        Ok(RoomID::new(id))
    }
}

impl Display for RoomID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut id = self.value;
        let mut s = ['A'; Self::CHAR_LENGTH];
        for i in (0..Self::CHAR_LENGTH).rev() {
            if id == 0 {
                break;
            }
            let c = (id % 26) as u8 + b'A';
            s[i] = c as char;
            id /= 26;
        }
        write!(f, "{}", s.iter().collect::<String>())
    }
}

impl<'de> serde::Deserialize<'de> for RoomID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <&str>::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for RoomID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAX: RoomID = RoomID {
        value: 26_u32.pow(RoomID::CHAR_LENGTH as u32) - 1,
    };

    #[test]
    fn test_parse() {
        assert_eq!("AAAAAA".parse::<RoomID>(), Ok(RoomID { value: 0 }));
        assert_eq!("AAAAAB".parse::<RoomID>(), Ok(RoomID { value: 1 }));
        assert_eq!("AAAAAZ".parse::<RoomID>(), Ok(RoomID { value: 25 }));
        assert_eq!("AAAABA".parse::<RoomID>(), Ok(RoomID { value: 26 }));
        assert_eq!("AAAABB".parse::<RoomID>(), Ok(RoomID { value: 27 }));
        assert_eq!("ZZZZZZ".parse::<RoomID>(), Ok(MAX));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            "AAAAA".parse::<RoomID>(),
            Err(RoomParseError::InvalidLength)
        );
        assert_eq!(
            "AAAAAAAA".parse::<RoomID>(),
            Err(RoomParseError::InvalidLength)
        );
        assert_eq!(
            "AAsdfAAAA".parse::<RoomID>(),
            Err(RoomParseError::InvalidLength)
        );
        assert_eq!("AAAAAa".parse::<RoomID>(), Err(RoomParseError::InvalidChar));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", RoomID { value: 0 }), "AAAAAA");
        assert_eq!(format!("{}", RoomID { value: 1 }), "AAAAAB");
        assert_eq!(format!("{}", RoomID { value: 25 }), "AAAAAZ");
        assert_eq!(format!("{}", RoomID { value: 26 }), "AAAABA");
        assert_eq!(format!("{}", RoomID { value: 27 }), "AAAABB");
        assert_eq!(format!("{}", MAX), "ZZZZZZ");
    }

    #[test]
    fn test_display_parse() {
        for i in 0..1000 {
            let id = RoomID { value: i };
            let str = format!("{}", id);
            assert_eq!(str.parse::<RoomID>(), Ok(id));
            assert_eq!(str.parse::<RoomID>().map(|r| r.value()), Ok(i));
        }
        let id = MAX;
        assert_eq!(format!("{}", id).parse::<RoomID>(), Ok(id));
    }

    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_serde() {
        assert_tokens(&RoomID { value: 0 }, &[Token::BorrowedStr("AAAAAA")]);
        assert_tokens(&RoomID { value: 1 }, &[Token::BorrowedStr("AAAAAB")]);
        assert_tokens(&RoomID { value: 25 }, &[Token::BorrowedStr("AAAAAZ")]);
        assert_tokens(&RoomID { value: 26 }, &[Token::BorrowedStr("AAAABA")]);
        assert_tokens(&RoomID { value: 27 }, &[Token::BorrowedStr("AAAABB")]);
        assert_tokens(&MAX, &[Token::BorrowedStr("ZZZZZZ")]);
    }
}
