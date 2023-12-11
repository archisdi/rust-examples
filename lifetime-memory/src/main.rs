#[allow(dead_code)]
#[allow(unused_imports)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

struct Company<'z> { // lifetime
    name: String,
    ceo: &'z Person
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name, state }
    }
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

impl Company<'_> { // lifetime
    fn get_ceo(&self) -> &Person {
        &self.ceo
    }
}

fn rc_demo() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("Happy".to_string()));
    let person = Person::new(name.clone(), state.clone());

    // thread safe because of Arc and Mutex
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());

    t.join().unwrap();
}

fn main() {
    // let v = vec![1, 2, 3];
    // let v2 = v;
    // println!("v[0] = {}", v[0]); // error: value borrowed here after move

    let u = 1;
    let u2 = u;
    println!("u = {}", u); // work because i32 is Copy of primitive type

    // the following code will not work because String is not Copy
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };
    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("v[0] = {}", v[0]);


    // -------------------------
    let mut a = 40;
    let b = &mut a;
    *b += 2;

    println!("a = {}", a);

    // -------------------------

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5); // error: cannot borrow `z` as mutable because it is also borrowed as immutable
    }

    // --- Lifetime
    // let boss = Person { name: String::from("Elon Musk") };
    // let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    // --- Rc
    rc_demo();
}
