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

fn main(){
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
    print_elements();
}