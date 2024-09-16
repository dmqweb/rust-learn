use std::num::ParseIntError;
use std::str::FromStr; //FromStr是标准库中的一个trait，用于将字符串转为其他类型

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}
//为Person结构体实现FromStr trait，用于将字符串转为其他类型
impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let name = match parts.next() {
            Some(name) if !name.is_empty() => name.to_string(),
            Some(_) => return Err(ParsePersonError::NoName),
            None => return Err(ParsePersonError::BadLen),
        };
        let age_str = match parts.next() {
            Some(age) => age,
            None => return Err(ParsePersonError::BadLen),
        };
        if parts.next().is_some() {
            return Err(ParsePersonError::BadLen);
        }
        let age = age_str.parse::<u8>().map_err(ParsePersonError::ParseInt)?;
        Ok(Person { name, age })
    }
}
fn main() {
    //实现FromStr之后就可以使用parse方法传入结构体作为泛型，进行类型转换。
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}
#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;
    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }
    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }
    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }
    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }
    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }
    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }
    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }
    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
