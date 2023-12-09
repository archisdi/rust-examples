mod sh;
#[allow(dead_code)]
#[allow(unused_imports)]

use std::mem;

// constant
const MEANING_OF_LIFE: u8 = 42; // no fixed address
static Z: i32 = 123; // fixed address

fn fundamental_data_types() {
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

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4; // +-*/
    println!("{}", a);

    a = a + 1;
    a -= 2; // -= += *= /= %=
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // powi for integer, powf for float
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    let x = 5;
    let x_is_5 = x == 5; // true
}

fn scope_and_shadowing() {
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("outside, b = {}", b); // error: cannot find value `b` in this scope
}

fn control_flow() {
    let temp = 25;

    if temp > 30 {
        println!("really hot outside");
    }
}

fn main() {
    fundamental_data_types();
    operators();
    scope_and_shadowing();
    sh::stack_and_heap();
}
