// region: Structs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right:
            Point {
                x: bottom_right_x,
                y: bottom_right_y,
            },
    } = rect;

    let a = (top_left_x - bottom_right_x).abs();
    let b = (top_left_y - bottom_right_y).abs();
    a * b
}

fn square(top_left: Point, a: f32) -> Rectangle {
    let Point { x: top_x, y: top_y } = top_left;
    let bottom_right = Point {
        x: (top_x + a),
        y: (top_y - a),
    };
    Rectangle {
        top_left,
        bottom_right,
    }
}
// endregion

// region: Enums
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("presses '{c}'"),
        WebEvent::Paste(s) => println!("pasted \"{s}\""),
        WebEvent::Click { x, y } => println!("clicked at x={x} y={y}"),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, a: i32, b: i32) -> i32 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
        }
    }
}

#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Three,
}

#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

mod linked_list;
use linked_list::main as linked_list_test;
use linked_list::List;
// endregion

fn main() {
    // region: Structs
    let area = rect_area(Rectangle {
        top_left: Point { x: 0.5, y: -1.0 },
        bottom_right: Point { x: 2.0, y: 3.0 },
    });
    println!("area = {area}");
    println!();

    let corner = Point { x: -2.25, y: 4.75 };
    let a: f32 = 3.0;
    let square = square(corner, a);
    println!("square: {:?}", square);
    // endregion
    println!();
    // region: Enums
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!();
    let (a, b) = (2, 5);
    println!("Add: {}", Operations::Add.run(a, b));
    println!("Subtract: {}", Operations::Subtract.run(a, b));

    println!();
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are 0x{:06x}", Color::Red as i32);
    println!("violets are 0x{:06x}", Color::Blue as i32);
    // endregion
    println!();
    // region: Linked list
    println!("Linked list");
    linked_list_test();
}
