use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Keywords {
    Position,
}

impl FromStr for Keywords {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "position" => Ok(Self::Position),
            _ => Err(()),
        }
    }
}

impl Keywords {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Position => "position",
        }
    }

    pub fn options() -> Vec<&'static str> {
        vec!["position"]
    }
}
