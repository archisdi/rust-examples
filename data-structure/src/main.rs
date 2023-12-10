#[allow(dead_code)]
#[allow(unused_imports)]

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 } // struct
}

fn structures() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p1, end: p2 };
    println!("my line starts at ({}, {}), ends at ({}, {})", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}

fn enumns() {
    let c: Color = Color::RgbColor(10, 0, 0);
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor { cyan: _, magenta: _, yellow: _, black: 255 } => println!("black"),
        Color::CmykColor { cyan: c, magenta: m, yellow: y, black: k } => println!("cmyk({}, {}, {}, {})", c, m, y, k)
    }
}

fn main() {
    structures();
    enumns();
