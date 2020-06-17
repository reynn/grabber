//! Detail the listing feature of Reddit

error_chain! {
    errors {
        TypeParseError(id: String) {
            description("Provided id does not have a valid type")
            display("{} has an unknown or invalid type", id)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub anchor: Option<Anchor>,
    pub batch_size: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            batch_size: 20,
            anchor: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Anchor {
    Before(String),
    After(String),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    Comment(String),
    Account(String),
    Link(String),
    Message(String),
    Subreddit(String),
    Award(String),
}

impl Type {
    pub fn parse(full_type_id: &str) -> Result<Self> {
        let mut split_type = full_type_id.split('_');
        let type_prefix = split_type.next().unwrap();
        let type_id = split_type.next().unwrap();

        println!("prefix: {:?}", type_prefix);
        match type_prefix {
            "t1" => Ok(Self::Comment(String::from(type_id))),
            "t2" => Ok(Self::Account(String::from(type_id))),
            "t3" => Ok(Self::Link(String::from(type_id))),
            "t4" => Ok(Self::Message(String::from(type_id))),
            "t5" => Ok(Self::Subreddit(String::from(type_id))),
            "t6" => Ok(Self::Award(String::from(type_id))),
            _ => Err(ErrorKind::TypeParseError(String::from(full_type_id)).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("t1_llq9kx", Some(Type::Comment(String::from("llq9kx"))), false ; "Type::Comment('llq9kx')")]
    #[test_case("t2_v9dlmt", Some(Type::Account(String::from("v9dlmt"))), false ; "Type::Account('v9dlmt')")]
    #[test_case("t3_15bfi0", Some(Type::Link(String::from("15bfi0"))), false ; "Type::Link('15bfi0')")]
    #[test_case("t4_yr9wki", Some(Type::Message(String::from("yr9wki"))), false ; "Type::Message('yr9wki')")]
    #[test_case("t5_8u7w12", Some(Type::Subreddit(String::from("8u7w12"))), false ; "Type::Subreddit('8u7w12')")]
    #[test_case("t6_s8t010", Some(Type::Award(String::from("s8t010"))), false ; "Type::Award('s8t010')")]
    #[test_case("t0_s810", None, true ; "Invalid type")]
    fn type_parse(s: &str, expected: Option<Type>, should_fail: bool) {
        let parsed_result = Type::parse(s);

        match parsed_result {
            Ok(parsed_type) => {
                println!("Type: {:?}", parsed_type);
                assert_eq!(parsed_type, expected.unwrap());
            }
            Err(parse_error) => assert!(
                should_fail,
                format!("Failed to parse provided ID: {} got error {}", s, parse_error)
            ),
        }
    }
}
