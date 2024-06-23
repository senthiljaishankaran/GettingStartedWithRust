fn main(){
    println!("Differencer b/t mut keyword and let for changing values");

    // declaring a value using mut keyword
    let mut num: u32 =5;
    println!("value of num with mut keyword before changing :{:?}",num);

    // declaring a variable using only let which creates a string slice
    let num2:u32 =8;
    println!("value of num2 without mut keyword before changing :{:?}",num2);

    // changing teh value of num created using mut keyword
    // can be done simple as that of below
    num =10;
    println!("value of num with mut keyword after changing :{:?}",num);

    // but we can't change the value of the string slice directly
    // it will through immutable error, so we need to use it with let keyword
    let num2: i32 =15;
    println!("value of num without mut keyword before changing :{:?}",num2);
    // major difference over here is using let keyword will reassign the value of string slice
    // where mut makes the variable flexible to be changed in the code later

}