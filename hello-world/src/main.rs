#[allow(dead_code)]
#[allow(unused_imports)]

use std::mem;

fn main() {
    let a: u8 = 123; // 8-bit unsigned integer, 0 to 255, immutable
    println!("a = {}", a);

    let mut b: i8 = 0; // 8-bit signed integer, -128 to 127, mutable
    println!("b = {} before", b);

    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789; // 32-bit signed integer, -2,147,483,648 to 2,147,483,647, type inferred from context
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {} after modification", c);

    // i8 u8 i16 u16 i32 u32 i64 u64, ...

    // usize isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE754
    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64 by default
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g: bool = false; // or true
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
