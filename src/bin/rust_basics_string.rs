fn main(){

    println!("Creating a new String");
    println!("Difference between creating string using new() and from()");

    // this is how we can create a new string in rust
    // this method creates a empty string and allocates minimal memory on heap for basic structure
    let mut new_string_new: String = String::new();
    // this is how we can utilize the created string using push_str or push method
    // if the below line is done for a newly created string it will raise a warning like below
    // warning: value assigned to `new_string_new` is never read
    new_string_new.push_str("New String created");
    println!("Created String using new() : {:?}",new_string_new);

    // Takes ownership of a string slice (often from a string literal) and creates a new String object from it.
    // Copies the content of the string slice into a newly allocated memory on the heap
    // This is suitable when you want to work with a pre-defined string value.
    let new_string_from: String = String::from("New String Modified");
    println!("Created String using from() : {:?}",new_string_from);
    
    println!("Diff b/t string and str");
    // even if  we did not declare the string variable to &str 
    // rust compiler automatically stores the variable as string in heap and creates a reference pointer to it 
    let any_string: &str = "Senthil";

    // here is where the toowned() comed in handy that when you want to own the data we can use it
    println!("Name is: {:?}",any_string);
    // assigning a immutable variable another value will generate error
    // like any_string = "Kumar"
    // either we can use the toowned() method and make a clone 
    // Creates owned data from borrowed data, usually by cloning, but it will also change the type to string from str
    // String is dynamically allocated string type that holds actual charater of the string meaning ownership and lives on heap memory
    let mut new_any_string: String = any_string.to_owned();
    // still we can't modify the straight way
    // like this new_any_string = ""
    new_any_string.push_str(" Kumar");
    println!("Name is: {:?}",new_any_string);

}


