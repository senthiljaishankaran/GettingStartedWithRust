fn main() {
    println!("First Rust Code!");
    println!("");

    println!("Getting Started with Datatypes,variable and print line macro");
    // Datatypes in rust are
    let bol : bool = true; // normal boolean datatype
    // "{:?}" or "{}" are called literals or token indicate that we take external value and include it in macro
    // "{:?}" use this when you need a debug friendly output in console
    // "{}" use this when you need a user friendly  human readable output in console
    println!("Printing Boolean type:{:?}", bol);
    let int : i32 = 100; // normal integer datatype here i32 means integer of 32bit
    println!("Printing Integer type:{:?}", int);
    let character : char = 'j'; // normal character datatype
    println!("Printing Character type:{:?}", character);
    let float : f64 = 1.50000; // normal float and double repesented with f64 meaning float with 64 bit
    println!("Printing Float type:{:?}", float);

    // There are two types of string in rust 
    // 1. str
    let mut st: &str = "Hi";
    println!("Printing string type:{:?}", st);
    // 2. String
    let str2 : String = "senthil".to_owned();
    st = "Hello";
    println!("Printing Two values:{} {}", str2,st);
    // printing two value in parallel
    let str3:String = "Kumar".to_owned();
    println!("Printing Name:{} {}",str2,str3);

    println!("My name is {:?}",str2);
    println!("");

    println!("Getting Started with flow control");
    println!("If else:");
    let num : i32 = 119;
    if num >=100{
        println!("Three digit number")
    }else if num <100 && num >=10 {
        println!("Two digit number")
    }else{
        println!("Single digit number")
    }
    
    println!("");
    println!("Printing loop:");
    let mut num2:i32 = 0;
    loop {
        if num2 == 5{
            break;
        }
        println!("{:?}",num2);
        num2 = num2+1;
    }

    println!("");
    println!("printing while loop:");
    let mut num3:i32 = 0;
    while num3!=5{
        println!("{:?}",num3);
        num3 = num3+1;
    }
}
