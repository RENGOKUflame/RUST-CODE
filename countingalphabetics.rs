use std::io::{self, Write};

fn main(){
    print!("Write Your Pararaph:\n\t\t     ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    let input1 = input.split_ascii_whitespace().count() ;
    println!("The Total Words are {input1}");
    let input2 = input.chars().filter(|&c| c.is_alphabetic()).count();
    println!("The Total Aplhabets are {input2}");
    if  input2 >200 && input2 <300 {
           println!("The Paragraph is sufficient..");
    }
    else {
        println!("Ok");
    }

}
