use std::io;

fn main(){
    println!("Getting input from user and printing");
    println!("May I Know Your Please!");
    // in order to get input from user we first need to define a string on which we will get the value from user
    let mut user_input = String::new();
    // if we did not handle the error expected with the stdin() then it ill show us the warning like below
    // this `Result` may be an `Err` variant, which should be handled
    // it should be handled with expect() method
    io::stdin().read_line(&mut user_input).expect("Failed to Read input");

    // here i am using trim() to remove the unwanted spaces or \n created by rust compiler
    let name = user_input.trim();

    if name.trim().is_empty(){
        println!("Name cannot be empty");
    }else{
        println!("Welcome, Mr.{:?}",name);
    }

}