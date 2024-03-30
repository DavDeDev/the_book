fn main() {
    // let v: Vec<i32> = Vec::new();
    // This automatically infers the type of Vec<i32>
    let v = vec![1, 2, 3];

    // To update a vector, mark it as mutable
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector v is equal to {:?}",v);

    // There are 2 ways to reference a value in a vector:

    // 1. Using the index
    let third_element: &i32 = &v[2];
    println!("The third element is {}", third_element);

    // 2. Using the get method

    // let third: Option<&i32> = v.get(2);
    // match third{
    //     Some(third)=> println!("The third element is {}", third),
    //     None => println!("There is no third element"),
    // }

    if let Some(third_element) = v.get(2){
        println!("The third element is {}", third_element);
    }

    // What happens if we try to access an element that doesn't exist?
    // This will cause the program to panic
    // let does_not_exist = &v[100];

    let does_not_exist = v.get(100);

    // ========== Ownership and Borrowing ==========
    // third_element is an immutable reference to the third element in the vector

    // v is a mutable reference to the vector ==> We can't have a mutable reference and an immutable reference in the same scope
    v.push(6);

    // This will cause an error because we keep into scope an immutable reference to the third element in the vector
    // println!("Vector v is equal to {third_element}");

    // ========== Iterating over the values in a vector ==========
    let v = vec![100, 32, 57];
    for i in &v{
        // we can't add a value to the vector while iterating over it
        println!("{i}");
    }

    // ========== Iterating over mutable references ==========
    let mut v = vec![100, 32, 57];
    for i in &mut v{ // for holds a mutable reference to the vector
        // we can modify the value of the vector while iterating over it but we can't insert/delete new values
        *i += 50;
    }

    // ========== Using an enum to store multiple types ==========

    // Vectors only store values of the SAME TYPE.

    // We can use an enum to store values of different types in a vector

    // For example, we can create an enum to store different types

    enum DifferentTypes{
        Integer(i32),
        String(String),
        Float(f64),
    }
    let vec = vec![
        DifferentTypes::Integer(5),
        DifferentTypes::String(String::from("Hello")),
        DifferentTypes::Float(3.14),
    ];

    // This doesn't work.
    // let vec = vec![
    //     5,
    //     String::from("Hello"),
    //     3.14,
    // ];
    
    // Rust has to know what types will be in the vector at compile time to be able to ALLOCATE ENOUGH MEMORY on the heap.

} // v goes out of scope and is freed here
