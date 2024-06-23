fn main(){
    println!("Referencing");

    // creating a mutable variable
    let mut og_value:i32 = 200;
    // referencing the mutable variable
    // this will create a pointer in stack called ref_value and points it to the og_value at heap
    // so there will be no new heap memory allocation done which makes the less garbage value available
    // still the garbage collectoris not availabel in rust though
    let ref_value = &og_value;

    // lets print and see the value
    println!("value of og_value: {:?}",og_value);
    println!("value of ref_value: {:?}",ref_value);

    println!("Problems in referencing");
    // let's say i change the value of the og_value and see what happens since it is mutable
    og_value = 150;

    // lets print and see the value again
    println!("value of og_value: {:?}",og_value);
    // if i try to do like this i will create a error stating
    // cannot assign to `og_value` because it is borrowed
    // tbis happens only if i try to print the value of borrowed variable
    // println!("value of ref_value: {:?}",ref_value);
    // explanation for this error is when we change the value of the variable it will destry the heap space that it had earlier
    // and creates a new heap space and stores the value, so the ref_value that has its pointer pointing to the that space is 
    // no longer available so it will be pointing towards nothing

    // lets see how the a value passed on from one function to other will affect the referencing
    let  og_string: String = String::from("Senthil");
    println!("Value of og_string is:{:?}",og_string);

    let ref_string = &og_string;
    println!("Value of ref_string is :{:?}",ref_string);

    // passing the value onto the function to change its scope thus changing the heap memory allocation
    destroy(og_string);
    
    // if i try to print the ref-string after passing through the function will produce error at the passing function area
    // stating cannot move out of `og_string` because it is borrowed
    //println!("Value of  ref_string after the og_string is passed to a function:{:?}",ref_string);

    println!("Static value");
    // the static object's life span is duration of the entire run time of code
    // it wont change due to free up of heap memory when it is re-assigned
    // or if the scope of the object changes which changes the heap memory allocation address
    // so the above error we faced can be rectified
    // this will produce error as it will expect the string to be either &str which will be only a reference
    // and it will try to say that the size of the variable cannot be determined at the compile time
    // we can either cahnge the type to &str or induce with tthe static keyword meaning the value will not change
    // so the size will known by the compiler at the compile time itself
    let strn : &'static str = "Kumar";
    let strn1: &&str = &strn;
    // Two ampersands together create a double reference. So, &&str is a reference to another reference to a string slice.
    // now lets try to change the scope using the destroy function
    destroy(strn.to_string()); // here the scope changed but and new heap is created but still it was not done on the original heap value
    // see now the scope is changed but still the reference works this is because we defined variable as static
    println!("{:?}",strn1)
    
}

fn destroy(string:String){
    println!("Changing the scope changes heap allocation: {:?}",string);
}