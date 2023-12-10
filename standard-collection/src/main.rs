#[allow(dead_code)]
#[allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);

    let idx: usize = 0;
    a[idx] = 0;
    println!("a[0] = {}", a[idx]);

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }
    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop(); // Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() { // remove all elements reversely
        println!("{}", x);
    }
}

fn hashmaps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("Triangle"), 3);
    shapes.insert(String::from("Square"), 4);
    shapes.insert("Square".into(), 5);

    println!("a square has {} sides", shapes["Square".into()]);

    shapes.entry("Circle".into()).or_insert(1);

    // update based on old value, by reference
    {
        let actual = shapes.entry("Circle".into()).or_insert(2);
        *actual = 0;
    }

    // for (key, value) in &shapes {
    //     println!("{}: {}", key, value);
    // }
    println!("{:?}", shapes);
}

fn hashsets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("we added vega!");
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("we removed delta");
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!("is {:?} a subset of {:?}? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("union: {:?}", _2_8.union(&_6_10));
}

fn iterators() {
    // iterator moves the value, rather than borrowing, so it can't be used again

    let mut vec = vec![3, 2, 1];
    for x in &vec { // only read, not modify so no need to be mutable
        println!("{}", x);
    }

    for x in &mut vec {
        *x += 1;
        println!("{}", x);
    }

    let mut vec2 = vec![1, 2, 3]; // macro to create vector
    let mut vec2_iter = vec2.iter(); // immutable iterator
    println!("{:?}", vec2_iter.next());
    println!("{:?}", vec2_iter.next());
    println!("{:?}", vec2_iter.next());
    println!("{:?}", vec2_iter.next());

    for x in vec2.iter().rev() { // reverse iterator
        println!("{}", x);
    }

    for x in vec2.iter_mut() { // mutable iterator 
        *x += 1;
        println!("{}", x);
    }

    let vec3 = vec![1, 2, 3];
    let vec3_iter = vec3.into_iter();
    for x in vec3_iter {
        println!("{}", x);
    }
}

fn main() {
    vectors();
    hashmaps();
    hashsets();
    iterators();
}
