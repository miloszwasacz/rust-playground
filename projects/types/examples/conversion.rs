// region: From & Into
use std::convert::From;

#[derive(Debug)]
struct Number {
    #[allow(dead_code)]
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}
// endregion

// region: TryFrom & TryInto
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}
// endregion

// region: String conversion
use std::str::FromStr;

#[derive(Debug)]
struct SmallNumber {
    #[allow(dead_code)]
    value: u8,
}

// NOTE Common implementation of parsing an object to string is to implement Display (see **Display** in *hello-world* project)
impl FromStr for SmallNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u8>();
        match parsed {
            Ok(value) => {
                if value < 10 {
                    Ok(SmallNumber { value })
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}
// endregion

fn main() {
    // region: From & into
    println!("From & Into");
    let int = 32;

    let number_from = Number::from(int);
    println!("Number from is {number_from:?}");

    let number_into: Number = int.into();
    println!("Number into is {number_into:?}");
    // endregion
    println!();
    // region: TryFrom & TryInto
    println!("TryFrom & TryInto");

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("All assertions correct!");
    // endregion
    println!();
    // region: String conversion
    println!("String conversion");
    let parsed: i32 = "5".parse().unwrap();
    println!("parsed = {parsed:?}");
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("turbo parsed = {turbo_parsed:?}");
    for custom_val in ["2", "20"] {
        let custom = custom_val.parse::<SmallNumber>();
        match custom {
            Ok(n) => println!("Parsed \"{custom_val}\" to \"{n:?}\""),
            Err(_) => println!("Error while parsing \"{custom_val}\" to SmallNumber"),
        }
    }
    // endregion
}
