use std::io::Error;

fn main() {
    let mut numbers: Vec<i32> =  vec![1,2,50,5,4,30,90,123,2];
    match largest(&numbers) {
        Ok(num) => println!("Max number is: {}",num),
        Err(_) => println!("Err")
    }

}


fn largest(list_of_numbers:&Vec<i32>)-> Result<&i32,&str>
{
    let mut max = list_of_numbers.get(0).ok_or("err")?;

    for number in list_of_numbers {
        if number > max {
            max = number;
        }
    }
    Ok(max)
}
