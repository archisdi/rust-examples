#[allow(dead_code)]
#[allow(unused_imports)]

use std::mem;
use std::fmt::Result;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Neg};
use std::cmp::{PartialEq};

struct Point<T = f64> {
    x: T,
    y: T
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1
}

fn product(x: i32, y: i32) -> i32 {
    x * y // no semicolon to return value
}

fn say_hello() {
    println!("hello");
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: i32) -> impl Fn(i32) -> bool { // function that returns a closure/anonymous function
    move |y| y > limit
}

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} * {} = {}", a, b, p);

    let p1 = Point { x: 1.0, y: 2.0 };
    println!("distance from origin = {}", p1.distance_from_origin());

    let f = say_hello;
    f();

    let plus_one = |x: i32| -> i32 { x + 1 }; // closure, anonymous function

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| { // type inference for parameters and return type
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));

    let plus_three = |x: &mut i32| { *x += 3 }; // closure with mutable reference
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    // higher order functions
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit); // closure

    for i in 0.. {
        let isq = i * i;

        // if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    // functional approach
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("functional sum = {}", sum2);
}

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Self {
        Human { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Self {
        Cat { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

// ---

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> { // Vec<i32> is the receiver
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self { result += *x; }
        result
    }
}

// ---

trait Shape {
    fn area(&self) -> f64;
}

trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result;
}

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Debug for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Circle {{ radius: {} }}", self.radius)
    }
}

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape + Debug>(shape: T) {
fn print_info<T>(shape: T) where T: Shape + Debug {
    println!("The area is {}", shape.area());
}

// --- INTO ---

struct Person {
    name: String
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person { // generic function to accept any type that can be converted into a String
        Person { name: name.into() }
    }
    fn new2<S>(name: S) -> Person where S: Into<String> {
        Person { name: name.into() }
    }
}

// --- DROP ---

struct Creature {
    name: String
}

impl Creature {
    fn new(name:  &'static str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature { // called when the object goes out of scope
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

// --- OPERATOR OVERLOADING ---

#[derive(Debug, Eq, PartialOrd)] // another way to derive traits
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T> where T: Add<Output = T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::<T> {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output = T> {
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex::<T> {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> PartialEq for Complex<T> where T: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

// --- STATIC & DYNAMIC DISPATCH ---

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

fn print_it<T: Printable>(z: T) { // static dispatch, monomorphization
    println!("{}", z.format());
}

fn print_it_dynamic(z: &dyn Printable) { // dynamic dispatch, will be slower but more flexible
    println!("{}", z.format());
}

fn traits() {
    // let h = Human { name: "John" };
    let h = Human::create("John");
    h.talk();

    // let c = Cat { name: "Misty" };
    let c = Cat::create("Misty");
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());

    let c = Circle { radius: 2.0 };
    print_info(c);

    // --- INTO ---
    let john = Person::new("John");

    let name = "Jane".to_string();
    let jane = Person::new(name);

    // --- DROP ---
    let goblin = Creature::new("Jeff");
    println!("Game proceeds");

    // --- OPERATOR OVERLOADING ---
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(3, 4);
    // println!("{:?}", a + b);
    a += b;
    println!("{:?}", -a); // -a is the same as a.neg()

    // --- STATIC & DYNAMIC DISPATCH ---
    let a = 123;
    print_it(a);

    let b = "hello".to_string();
    print_it_dynamic(&b);
}

fn main() {
    // functions();
    traits();
}
