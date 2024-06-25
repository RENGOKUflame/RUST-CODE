     //  Sum of array elements............

fn main(){
    let list = [45,45];
    for numbers in list{
        println!("Elements in array are:{}",numbers);
    }
    let mut sum: i64= 0;
    for elements in list{
        sum+=elements;
    }
    println!("\n");
    println!("The sum of arrays is {}",sum);
}

