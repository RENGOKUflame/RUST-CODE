use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guess The number..");
    println!("Remeber You Have Only 3 Attempts...");
    println!("Enter Your Starting Value:");
    io::stdout().flush().unwrap(); 
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Failed to read Line");
    let start : u32 = start.trim().parse().expect("Please Type Integer");
    println!("Enter Yur Ending Value:");
    io::stdout().flush().unwrap();
    let mut end= String::new();
    io::stdin().read_line(&mut end).expect("Failed to read line");
    let end : u32 = end.trim().parse().expect("Please Type Integer");
    let secret = rand::thread_rng().gen_range(start..=end);
    for _ in 1..=3 {
        println!("Enter Your Guess:");
        io::stdout().flush().unwrap();
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess :u32= match guess.trim().parse()   {
            Ok(num)=>num,
            Err(_)=>{
                   println!("Please Enter Integer..:");
                   continue;          
            }

    };
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
            println!("You Win...");
            print!("\n");
            println!("Select Your reward");
            println!("Loot Box 1\nLoot Box 2\nLoot Box 3");
            print!("Just Type Your Loot Box Number....");
            println!("\nEnter It Here:");
            io::stdout().flush().unwrap();
            let mut choice=String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line ");
            let choice :u32 = choice.trim().parse().expect("Please Type Integer");
            if choice == 1{
                print!("If You Can Change Your Mind.You Can Change Your Life");
            }
            else if choice == 2 {
                println!("Wake Up With Dtermination And Go To Bed With Satsifaction");
            }
            else if choice == 3 {
                println!("I Never Dreamed About Success.I Worked For It");
            }
            break;
        }   
    }
}

}
