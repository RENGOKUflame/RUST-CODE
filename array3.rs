        //subtractiono of array elements..............

fn main(){
    let list1 = [45,45];
    for numbers in list1{
        println!("Each element is :{}",numbers);
    }
    let mut sub = 0;
    for elements in list1{
        sub= elements - sub;
    }
    println!("\n");
    println!("The subtraction of array elememts is:{sub}");
}
