use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Rights {
    Administrator,
    Common,
}

impl ToString for Rights {
    fn to_string(&self) -> String {
        match self {
            Rights::Administrator => "ADMIN",
            Rights::Common => "USER",
        }
        .to_string()
    }
}

impl FromStr for Rights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADMIN" => Ok(Rights::Administrator),
            "USER" => Ok(Rights::Common),
            _ => Err(()),
        }
    }
}
