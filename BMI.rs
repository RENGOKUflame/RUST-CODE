use std::io;
fn main(){
println!("Enter Your Height in Centi-Meters:");
let mut input1= String::new();
io::stdin().read_line(&mut input1).expect("Failed to read line");
let  num:f32=input1.trim().parse().expect("Please Enter Height In Centi-Meters");
let  num1 = num/100.0;
println!("Enter Your Weight In Kilo-Grams");
let mut input2=String::new();
io::stdin().read_line(&mut input2).expect("Failed to read line");
let num2:f32=input2.trim().parse().expect("Please Enter Weight In Kilo-Grams");
let bmi = num2 / (num1*num1);
println!("Your BMI is {:.1}",bmi);
if bmi < 18.5 {
    println!("You are Considered Under-Weight...");
 }
else if bmi>=18.5 && bmi <25.0   {
    println!("You are Considered Normal Weight...");
}
else if bmi >= 25.0 && bmi <= 30.0  {
    println!("You are Considered Over-Weight...");
}
else {
    println!("You are Considered Obeses...");
}

}
