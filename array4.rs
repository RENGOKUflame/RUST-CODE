           // Multiplication of array elemnts.............    
fn main()
{
    let list3 = [20,5];
    let mut multi=1;
    for numbers in list3{
        println!("Each element is {numbers}");
    } 
    for numbers in list3{
              multi=numbers*multi;
    }
    println!("Final multiplication is {multi}");
}
