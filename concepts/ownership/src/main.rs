use std::rc::Rc;

fn print_elements(){
    let mut elements = vec![1,5,6,11,3,7]; // elements are allocated on heap
    for i in 6..20{
        let next = elements[i-3] + elements[i-2];
        elements.push(next);
    }
    println!("Elements(1..20) = {:?}", elements); 
} //elements vector goes out of scope and gets dropped here

struct Person {
    name: String,
    skill: String,
    age: u32
}

struct Label { number: u32 }

fn print(l: Label) { println!("STAMP: {}", l.number); }


fn main(){

    let rust: Rc<String> = Rc::new("Rustacean".to_string());
    let s = rust.clone();
    let z = rust.clone();
    assert!(rust.contains("Rust"));
    assert_eq!(s.find("cean"), Some(5));
    println!("I am a {}", z);


    let l = Label { number: 3 };
    println!("My label number is: {}", l.number);
    print(l);


    let mut programmers = Vec::new();
    programmers.push(Person {
        name: "Saman".to_string(),
        skill: "Rust".to_string(),
        age: 25
    });
    programmers.push(Person {
        name: "Arsmith".to_string(),
        skill: "C++".to_string(),
        age: 28
    });
    for programmer in &programmers{
        println!("{} codes in {} at {}",programmer.name,programmer.skill,programmer.age);
    }

    // let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = &s;
    
    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string();
    println!("{:?}, {}",s,t);

    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
        println!("v:{:?}",v);
    }
    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().unwrap();
    println!("fifth:{fifth}");
    assert_eq!(fifth, "105");
    // 2. Move a value out of the middle of the vector, and move the last
    // element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");
    println!("second:{second}");
    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    println!("third:{third}");
    assert_eq!(third, "103");
    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);



    print_elements();
}