use std::io;

fn greatest_common_divisor(mut num1: u32, mut num2: u32)->u32{
    
    assert!(num1 != 0 && num2 != 0);
    while num1 != 0 {
        if num1 < num2 {
            let another_num = num1;
            num1 = num2;
            num2 = another_num;
        }
        num1 = num1 % num2;
    }
    num2
}

fn main() {
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("failed to read line");
    let num1: u32 = num1.trim().parse().expect("Input an Integer");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("failed to read line");
    let num2: u32 = num2.trim().parse().expect("Input an Integer");
    let result = greatest_common_divisor(num1,num2);
    println!("Greatest common divisor of {} and {} is {}:",num1,num2,result);
}
