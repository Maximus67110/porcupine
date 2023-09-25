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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid_keyword() {
        assert_eq!(Keywords::from_str("position"), Ok(Keywords::Position));
    }

    #[test]
    fn test_from_str_invalid_keyword() {
        assert_eq!(Keywords::from_str("invalid"), Err(()));
    }

    #[test]
    fn test_to_str() {
        assert_eq!(Keywords::Position.to_str(), "position");
    }

    #[test]
    fn test_options() {
        assert_eq!(Keywords::options(), vec!["position"]);
    }
}
