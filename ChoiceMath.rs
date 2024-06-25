// This program is a even odd to print in series
//............................................................
#[allow(unused_variables)]
//use std::io;
/*
fn main()    
{
    //choice wala section ka code................
    println!("Enter your choice:\n1(EVEN NUMBERS)\n2(ODD NUMBERS)\n3(ADDITION)\n4(SUBTRACTION)\n5(MULTIPLICATION)\n6(FACTORAIL)");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let  number3:i8=choice.trim().parse().expect("Failed to read line");
    println!("Your choice is:{}",number3);
    //even number print karana ka liya ha.................
    if number3 ==1{
        println!("Enter starting value:");
        let mut starting=String::new();
        io::stdin().read_line(&mut starting).expect("Failed to read line");
        let mut number1:i32=starting.trim().parse().expect("Failed to read line");
        println!("Your starting value is:{}",number1);
        //yaha se ending value ka case ha..................
        println!("Enter ending value:");
        let mut ending = String::new();
        io::stdin().read_line(&mut ending).expect("Failed to read line");
        let  number2:i32=ending.trim().parse().expect("Failed to read line");
        println!("Your ending value:{}",number2);
        while number1 <= number2 {
         if number1 % 2 == 0{
            println!("Even number:{}",number1);
            number1+=1;
         }
            number1+=1;
        }
        //yaha se odd number print karana  kah liya ha....................    
    }
    else if number3==2{
        println!("Enter starting value:");
        let mut starting=String::new();
        io::stdin().read_line(&mut starting).expect("Failed to read line");
        let mut number1:i32=starting.trim().parse().expect("Failed to read line");
        println!("Your starting value is:{}",number1);
        println!("\n");
        //yaha se ending value ka case ha.........................
        println!("Enter ending value:");
        let mut ending = String::new();
        io::stdin().read_line(&mut ending).expect("Failed to read line");
        let  number2:i32=ending.trim().parse().expect("Failed to read line");
        println!("Your ending value:{}",number2);
    println!("\n");
    while number1 <= number2 {
       if number1 % 2 != 0{
        println!("Odd number:{}",number1);
        number1+=1;
    }
    number1+=1;
    } 
}
// yaha se addition wala case shuru ho jata ha..............
else if number3 == 3{
    println!("Enter numbers if you want to keep adding..\nType 'done' if you are finsih");
    let  mut sum = 0 ; 
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input =input.trim();
            if input.eq_ignore_ascii_case("done"){
                break;
            }
            match input.parse::<i32> ()   {
                Ok(num)=>{
                    sum=num+sum;
                    println!("Current sum is  = {} ",sum);
                }
                Err(_) =>{
                    println!("Invalid Input..");
                }  
                
                
        }
    }
    println!("Final sum:{}",sum);
}
//yaha se subtractio wala shuru ho jata ha..........................
else if number3==4  {
    println!("Enter numbers if you want to subtract.\nType 'done' if you want to finsh");
    let mut sub = 0 ;
    loop {
        let mut sub_num = String::new();
        io::stdin().read_line(&mut sub_num).expect("Failed to read line");
        let sub_num = sub_num.trim();
        if sub_num.eq_ignore_ascii_case("done"){
            break;
           }
           match sub_num.parse::<i32>(){
            Ok(num)=>{ 
                sub= num-sub ;
                println!("Current Sub {}",sub);
            }
               Err(_)=>{
                println!("invalid input..");
            }   
        }
    } 
    println!("Final Subtraction {}",sub); 
    
    //Yaha se multiplying wala shuru ho jata ha ....................        
}
else if  number3 ==5 {
    println!("Enter numbers if you want to keep on multiplying.\nType 'done' to finish");
    let mut  product =1;
        loop {
            
        let mut pro_input =String::new();
        io::stdin().read_line(&mut pro_input).expect("Failed to read line");
        let pro_input=pro_input.trim();
        if pro_input.eq_ignore_ascii_case("done"){
            break;
        } 
        match pro_input.parse::<i32>(){
            Ok(number)=>{
                product = number*product;
                println!("Current Product is {}",product);
            }
            Err(_)=>{
                println!("Invalid input...");
            }
        } 
        
    }
    println!("Final product is {}",product);   
        } 
        else if number3 == 6 {
            println!("Enter a number:");
            let mut factorial = String::new();
            io::stdin().read_line(&mut factorial).expect("Failed to read line");
            let  factorial:i64= factorial.trim().parse().expect("Failed to read line");  
            println!("Your number is {}",factorial);
            let mut product = 1 ;
            for x in 1..=factorial  {
                product*=x;
            }
            println!("The factorial of your number is {}",product);
        }   
        else {
            println!("Invalid choice....");
        } 
        
        
    }

    //this is the end of this program.........  
    //.....................................................................
   */  
