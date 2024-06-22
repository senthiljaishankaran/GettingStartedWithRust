fn main(){
    println!("Creating Vectors");
    /*  
    Vector is a fundamental data structure that represents a dynamically resizable array
    Advantages of Vector:
        vectors can grow or shrink at runtime. You can add or remove elements as needed
        Data contained by vector is owned by itself and stored in heap
        Vectors are generic, meaning they can hold elements of any data type 
        Vectors provide efficient random access to elements using indexing ([])
    */    

    // this is how we can create a vector using the new() in Vec
    // And in Vec<datatype> we can input the datatype on which we need to create a vector
    let mut numbers: Vec<i32> = Vec::new();
    // using push method to add element in the vector
    numbers.push(100);
    numbers.push(50);
    numbers.push(10);

    // elements in vector before pop and remove method
    for number in numbers.iter_mut(){
        println!("Vector elements are: {:?}",number)
    }

    // pop method
    // this method will remove the last element and return all the other elements in the list
    println!("Elements after using the pop and remove method");
    numbers.pop();

    // remove method will remove the element at the specified index
    numbers.remove(1);

    // Accessing the element using the index
    let first_number = numbers[0];
    println!("First number is {:?}",first_number);

    // Acessing using for loop and iter()
    for number in numbers.iter(){
        println!("Numbers are : {:?}",number);
    }

    println!("Creating Vector using macro");
    let mut number_new : Vec<i32> = vec![1,2,3,4,5];
    println!("New Numbers before drain {:?}",number_new);
    number_new.drain(1..3);
    println!("Number after drain: {:?}",number_new);

    // understanding difference between iter and iter_mut
    // using the iter we can only read the vectors and if we want to modify the elements then we need to use iter_mut
    println!("Diff b/t iter() and iter_mut()");
    println!("using iter()");
    for number in number_new.iter(){
        // using code like this will show error as will have have only reference to the elements
        // error will be like expected &i32 and found i32
        // number = number+1;
        println!("vectors are: {:?}",number);
    }

    for number in number_new.iter_mut(){
        // here it wont show like that because it is a mutable function also
        // below you would be seeing a star on left side of the variable meaning dereferencing
        // this dereferencing is an operation accesses the actual value stored at a memory location pointed to by a reference.
        *number +=1;
        println!("modified vectors are: {:?}",number);

        println!("Using len and capacity method");
        let mut length_number: Vec<i32> = vec![12,34,42,77];
        // here i have defined the vector where the length represents the number of elements
        // the number of elements may change so the length of elements will change
        // but the capacity is defined at the time declaration so even if the element is removed capacity remains same or increase but not decrease
        length_number.pop();
        let num_length = length_number.len();
        let capacity = length_number.capacity();
        println!("Length of the numbers is : {:?}",num_length);
        println!("Capacity of the number is : {:?}", capacity);
    }

}