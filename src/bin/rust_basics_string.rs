fn main(){

    println!("Creating a new String");
    // this is how we can create a new string in rust
    let mut new_string = String::new();
    // this is how we can utilize the created string usinh push_str or push method
    // if the below line is done for a newly created string it will raise a warning like below
    // warning: value assigned to `new_string` is never read
    new_string.push_str("New String created");
    println!("Created String : {:?}",new_string);
    new_string = String::from("New String Modified");
    println!("Created String : {:?}",new_string);
    
    println!("Diff b/t string and str");
    // we did not declare the string variable to &str 
    // rust compiler automatically stores the variable as string in heap and creates a reference pointer to it 
    let any_string = "Senthil";

    // here is where the toowned() comed in handy that when you want to own the data we can use it
    println!("Name is: {:?}",any_string);
    // assigning a immutable variable another value will generate error
    // like any_string = "Kumar"
    // either we can use the toowned() method and make a clone 
    // Creates owned data from borrowed data, usually by cloning, but it will also change the type to string from str
    // String is dynamically allocated string type that holds actual charater of the string meaning ownership and lives on heap memory
    let mut new_any_string = any_string.to_owned();
    // still we can't modify the straight way
    // like this new_any_string = ""
    new_any_string.push_str(" Kumar");
    println!("Name is: {:?}",new_any_string);

}


