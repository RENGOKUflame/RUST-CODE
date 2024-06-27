use std::io;
    fn main(){
    println!("Enter size of array:");
    let mut input1=String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let   num1:usize=input1.trim().parse().expect("Please type integer");
    let  mut  list:Vec<usize>=Vec::with_capacity(num1);
    for _ in 1..=num1{
           println!("Enter your numbers:");
           let mut input2=String::new();
           io::stdin().read_line(&mut input2).expect("Faield to read line");
           let  num2 : usize = input2.trim().parse().expect("Please type integer");
           list.push(num2);
    }
        print!("Your array is {:?}",list);
        println!("\n");
        reverse_array(&mut list);
}
fn reverse_array(list:&mut [usize]){
    let  len = list.len();
    print!("Length of array is {len}");
    print!("\n");
    for i in 0..len/2{
       list.swap(i,len-1-i);
    }
    println!("\n");
    println!("Reversed Array is as follows:{:?}",list)
}
