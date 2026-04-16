use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Hostname(String);

impl FromStr for Hostname {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hostname(s.to_owned()))
    }
}
