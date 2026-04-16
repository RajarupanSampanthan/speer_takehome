use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_with::DeserializeFromStr;

#[derive(DeserializeFromStr, Debug, Clone, PartialEq)]
pub struct IpV4Address {
    byte_rep: u32,
}

impl FromStr for IpV4Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<u8>, _> = s.split('.').map(|x| x.parse::<u8>()).collect();

        if parse_result.is_err() {
            return Err(String::from("Parse Error: Not u8"));
        }

        let values = parse_result.unwrap();

        if values.len() != 4 {
            return Err(String::from("Parse Error: Not 4 values"));
        }

        let byte_rep: u32 = values.into_iter().fold(0, |acc, x| (acc << 8) + x as u32);
        Ok(IpV4Address { byte_rep })
    }
}

impl Serialize for IpV4Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = (0..4)
            .map(|index| ((self.byte_rep) >> (8 * index)) & 255)
            .map(|x| x.to_string())
            .reduce(|acc, e| format!("{}.{}", acc, e))
            .unwrap();

        serializer.serialize_str(&s)
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_deserialize() {
        let expected = IpV4Address { byte_rep: 256 + 1 };

        let generated = IpV4Address::from_str("0.0.1.1").unwrap();

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_no_value_greater_than_256() {
        let result = IpV4Address::from_str("257.0.0.0");
        assert!(result.is_err());
        let result = IpV4Address::from_str("0.257.0.0");
        assert!(result.is_err());
        let result = IpV4Address::from_str("0.0.257.0");
        assert!(result.is_err());
        let result = IpV4Address::from_str("0.0.0.257");
        assert!(result.is_err());
    }

    #[test]
    fn test_4_sections() {
        let result = IpV4Address::from_str("0.0.0.0");
        assert!(result.is_ok());
        let result = IpV4Address::from_str("0.0.0.0.0");
        assert!(result.is_err());
    }
}
