use std::io;
fn main(){
        println!("Enter The Text You Want To Encrypt:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed To Rad Line");    
        let  input1  = input.trim().to_string();  
        println!("You Entered:{:?}",input1);
        let shifter = 3 ; 
        let encrypt = encryption(&input1,shifter );
        println!("The Encrypted Text will be: {encrypt}");

    }
fn encryption(a:&str,b:u8)-> String{
      a.chars()
      .map(|c|
            if c.is_ascii_alphabetic(){
                let base = if c.is_ascii_lowercase() {b'a'} else {b'A'}; 
                ((c as u8 - base + b) % 26 + base) as char
            }
            else{
                c
            }
        
        )
        .collect()
}
