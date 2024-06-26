#[allow(unused_variables)]
use std::io;
fn main(){ 
       println!("Enter your array size:");
       let  mut input1 = String::new();
       io::stdin().read_line(&mut input1).expect("Failed to read line");
       let  num1:usize = input1.trim().parse().expect("Please type the integer");
       println!("Your value is {num1}");
       let mut list:Vec<usize>= Vec::with_capacity(num1);
       for _ in 1..=num1{
                println!("Enter a number:");
                let mut input2=String::new();
                io::stdin().read_line(&mut input2).expect("Failed to read line");
                let num2:usize= input2.trim().parse().expect("Please Type Integer");
                list.push(num2);
       }
       println!("Array:{:?}",list);
       println!("\n");
       println!("The sum of your arrays is");
       let mut sum:usize=0;
       for elements in list{
                      sum=elements+sum;
       }
       println!("{sum} ");

       
} 
