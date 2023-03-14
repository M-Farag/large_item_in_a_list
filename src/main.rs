use std::io::Error;

fn main() {
    let numbers: Vec<i32> =  vec![1,2,50,5,4,30,90,123,2];
    match largest(&numbers) {
        Ok(num) => println!("Max number is: {}",num),
        Err(_) => println!("Err")
    }

    let numbers: Vec<f64> =  vec![1.90,1.675,1.5463,1.8604,1.423,1.30,1.901,1.123,1.2];
    match largest(&numbers) {
        Ok(num) => println!("Max number is: {}",num),
        Err(_) => println!("Err")
    }



}

// Generic
fn largest<T: std::cmp::PartialOrd>(list_of_numbers:&Vec<T>)-> Result<&T,&str>
{
    let mut max = list_of_numbers.get(0).ok_or("err")?;

    for number in list_of_numbers {
        if number > max {
            max = number;
        }
    }
    Ok(max)
}
