fn main() {
    hello_world();
    println!();
    string_format();
    println!();
    string_format_number();
    println!();
    string_format_len();
    println!();
    pi_exercise();
    println!();
    string_format_debug();
    println!();
    string_format_display();
    println!();
    string_format_display_formatting();
}

fn hello_world() {
    println!("Hello, world!");
    let x = 5 + 90 + 5;
    let y = "haha";
    println!("Number is \"{}\", text is \"{}\"", x, y);
}

fn string_format() {
    println!("{0} this is {1}, {1} this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}

fn string_format_number() {
    let number = 69420;
    println!("Base dec: {}", number);
    println!("Base bin: {:b}", number);
    println!("Base oct: {:o}", number);
    println!("Base hex: {:x}", number);
    println!("Base hex: {:X}", number);
    println!("Lower Exponential: {:e}", number);
    println!("Upper Exponential: {:E}", number);
}

fn string_format_len() {
    let num = 1;
    // println!("{number:>5}", number = num);
    println!("{num:>5}");
    println!("{num:0>5}");
    println!("{num:0>width$}", width = 3);
    println!("{num:a>3}"); // Works with every character

    // Alignment
    {
        let x = "x";
        println!("\n//alignment");
        println!("Hello {x:<5}!");
        println!("Hello {x:-<5}!");
        println!("Hello {x:^5}!");
        println!("Hello {x:>5}!");
        println!("Hello {x:->5}!");
        println!("//alignment");
    }

    // Float
    {
        println!("\n//float");
        let num1 = 1.0;
        let num2 = 1.5;
        let width: usize = 5;
        println!("{num1:0>width$}");
        println!("{num2:0>width$}");
        println!("//float");
    }

    // Pair
    {
        println!("\n//pair");
        let pair = (3, 'c');
        println!("{pair:?}");
        println!("{pair:#?}");
        println!("//pair");
    }
}

fn pi_exercise() {
    let pi = 3.141592;
    let result = format!("{} is roughly {:.*}", "Pi", 3, pi);
    println!("{result}")
}

// region: Debug
#[allow(dead_code)]
struct Unprintable(i32);

#[derive(Debug)]
struct Printable(i32);

#[derive(Debug)]
struct Nested(Printable);

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn string_format_debug() {
    println!("{:?} months", 12);
    println!(
        "{name:?} is the {:?} of a {:?}",
        "name",
        "person",
        name = "Adam"
    );
    // println!("{}", Unprintable(1));
    println!("{:?}", Printable(1));
    println!("{:?}", Nested(Printable(1)));
    let person = Person {
        name: "Adam",
        age: 32,
    };
    println!("{person:?}");
    println!("{person:#?}");
}
// endregion

// region: Display
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{v}")?;
            // write!(f, "{count}: {v}")?;
        }

        write!(f, "]")
    }
}

fn string_format_display() {
    let point = Point2D { x: 0.2, y: 1.5 };
    println!("{point}");
    println!("{point:?}");
    println!("{}", List(vec![1, 2, 3, 5]));
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

fn string_format_display_formatting() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
}
// endregion
