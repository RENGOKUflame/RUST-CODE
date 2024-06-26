#[allow(unused_variables)]
use std::io;
fn main(){
        println!("Enter your array size:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to read line");
        let num1:usize=input1.trim().parse().expect("Please type integer");
        let mut list:Vec<usize>=Vec::with_capacity(num1);
        for _ in 1..=num1{
            println!("Enter a number");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("Failed to read line");
            let num2:usize = input2.trim().parse().expect("Please type intger");
            list.push(num2);
        }  
        println!("Array is{:?}",list);
        println!("\n");
        println!("Multiplication of your array is");
        let mut product:usize= 1;
        for elements in list{
            product=elements*product;
        }
        println!("Final product is{product}");
}

