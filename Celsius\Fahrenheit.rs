use std::io::{self,Write};
fn main(){
    println!("Enter '1' to convert Celsius to Fahrenheit");
    println!("Enter '2' to convert Fahrenheit To Celsius");
    println!("Enter '3' to Terminate");
    io::stdout().flush().unwrap();
    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let number1 :i8 =choice.trim().parse().expect("Please type integer");
    if number1 == 1 || number1 == 2 || number1 == 3{
            println!("You Entered Right Choice");  
    }

        if number1 != 1 && number1 != 2 && number1 != 3{
            loop {            
                println!("You Entered Wrong Choice.:");
                io::stdout().flush().unwrap();
                let mut choice1=String::new();
                io::stdin().read_line(&mut choice1).expect("Failed to read line");
                let number1:i8=choice1.trim().parse().expect("Please Type Integer"); 
                if number1 == 1 || number1 == 2|| number1 == 3{
                               break;
                            }  
                                         }             
        }
       let number2 = number1; 
     if number2 == 1{
             println!("Converting Celsius To Fahrenheit");
             println!("Enter Celsius:");
             io::stdout().flush().unwrap();
             let mut input1=String::new();
             io::stdin().read_line(&mut input1).expect("Failed to read line");
             let celsius:f64 =input1.trim().parse().expect("Please type value");
             let con = ctof(celsius);
             println!("The Fahrenheit will be as follows:{:.2}",con);
              
     }
        else if number2 ==2{
        println!("Converting Fahrenheit To Celsius");
        println!("Enter Fahrenheit");
        io::stdout().flush().unwrap();
        let mut input2=String::new();
        io::stdin().read_line(&mut input2).expect("Failed toread line");
        let fah:f64=input2.trim().parse().expect("Please Type Value");
        let con1:f64 = ftoc(fah);
        println!("The Celsius will be as follows:{:.2}",con1);
     }
     else {
         print!("The Program Ends Here");
     }
}
fn ftoc(f:f64)->f64{
        (f-32.0)*0.5556
}
fn ctof(c:f64)-> f64{
        (c*1.8)+32.0
}
