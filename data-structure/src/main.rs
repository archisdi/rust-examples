#[allow(dead_code)]
#[allow(unused_imports)]

use std::mem;
mod pm;

struct Point<T = f64> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor { cyan: u8, magenta: u8, yellow: u8, black: u8 } // struct
}

// 32 bits
union IntOrFlow {
    i: i32,
    f: f32
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

fn process_value(iof: IntOrFlow) {
    unsafe {
        match iof {
            IntOrFlow { i: 42 } => {
                println!("meaning of life value");
            },
            IntOrFlow { f } => {
                println!("value = {}", f);
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFlow { i: 123 };
    iof.i = 234;
    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFlow { i:5 });
}

fn options() {
    let x = 3.0;
    let y = 2.0;

    // Option
    let result = if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    if let Some(z) = result {
        println!("z = {}", z);
    }

    while let Some(z) = result {
        println!("z = {}", z);
        break;
    }
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    if a != [1, 2, 3, 4, 5] { // cannot compare with different length
        println!("does not match");
    }

    let b = [1u16; 10]; // fill with 1u16, 10 elements
    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [ // 2x3 matrix
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    // iterate over matrix to print diagonal elements
    // for i in 0..mtx.len() {
    //     for j in 0..mtx[i].len() {
    //         if i == j {
    //             println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
    //         }
    //     }
    // }
    // arrays always have a fixed size, use slices instead if you need a dynamic size
}

fn use_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]); // by reference, value is not copied and can be modified
    println!("{:?}", data);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("sp = ({}, {})", sp.0, sp.1);
    println!("sp = ({}, {})", sp.0, sp.1);

    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined; // destructuring
    println!("c = {}, d = {}, e = {}, f = {}", c, d, e, f);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);
}

fn generics() {
    let a: Point<i32> = Point { x: 0, y: 0 };
    let b: Point = Point { x: 1.2, y: 3.4 }; // default type is f64
    // let my_line: Line<i32> = Line { start: a, end: b };
}

fn strings() {
    let s: &'static str = "hello there!"; // cannot be mutated nor dropped, lives for the entire program

    for c in s.chars().rev() { // reverse iterator over characters
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) { // Option, first character of string
        println!("first letter is {}", first_char);
    }

    // String - growable, heap-allocated data structure
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u: &str = &letters; // convert String to &str, dereference coercion

    // concatenation
    let z = letters + "abc"; // takes ownership of letters

    let mut abc = "Hello world".to_string(); // String from &str
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));

    // String formating
    let name = "John";
    let greeting = format!("Hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello_rust = format!("{}, {}", "Hello", "Rust");
    println!("{}", hello_rust);

    // positional arguments to format string
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    // named arguments
    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "James",
        last = "Bond"
    );
    println!("{}", info);

    // mixed positional and named arguments
    let mixed = format!(
        "{1} {} {last}",
        "Bond",
        first = "James",
        last = "Bond"
    );
    println!("{}", mixed);

}

fn main() {
    structures();
    enumns();
    unions();
    options();
    arrays();
    slices();
    tuples();
    pm::pattern_matching();
    generics();
    strings();
}