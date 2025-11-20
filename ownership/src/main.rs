

fn main() {
    what_is_ownership();
}

fn what_is_ownership() {
    // let x = [0, 1_000_000]; 
    // let y = x // this would copy the entire array into the stack, which is a lot of data.
    // instead we can use a pointer to the array on the heap.
    // let x = Box::new([0, 1_000_000]);
    // let y = x; // this copy the pointer of x rather than copying the entire array on to the stack.

    let x = Box::new(5); // x is a pointer to a value of 5 on the heap. It lives as long as the box does.

    // heap data is not tied to the stack frame.

    // rust does not permit manual memory management.
    // when a function is called, rust allocates memory on the stack for the
    // function's arguments and local variables. This is then deallocated when the function returns.

    // Rust does not allow you to free memory manually. As it could lead to undefined behavior.
    // Box deallocation principle (almost correct): If a variable is bound to a box, when Rust deallocates the variable’s frame,
    // then Rust deallocates the box’s heap memory.

    let a_num = 4; // 4 allocated into memory
    make_and_drop(); // 5 allocated onto the heap, then deallocated when the function returns as it's not needed anymore.
    // <- heap empty at this point

    // let a = Box::new([0; 1_000_000]);
    // let b = a; 
    // This would be invalid, because a and b would both point to the same heap memory.
    // When b goes out of scope, it would deallocate the heap memory, but a would still point to it.
    // This would be a dangling pointer.
    //
    // Instead we say that a has ownership of the heap memory.
    //
    // let b = a; // this would transfer the ownership of the heap memory to b.
    //
    // Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates the variable’s frame,
    // then Rust deallocates the box’s heap memory.

    // Boxes are used by Rust data structures1 like Vec, String, and HashMap to hold a variable number of elements.
    

    let name = String::from("John"); // name owns the heap memory for the string "John"

    let full_name = add_suffix(name); // full_name now owns the heap memory for the string "John Jr."
    // while name is no longer valid.

    // At L1, the string "John" has been allocated on the heap. It is owned by name.
    //
    // At L2, the function add_suffix(name_to_extend) has been called.
    // This moves ownership of the string from name to name_to_extend.
    // The string data is not copied, but the pointer to the data is copied.
    //
    // At L3, the function name_to_extend.push_str(" Jr.") resizes the string’s heap allocation.
    // This does three things. First, it creates a new larger allocation.
    // Second, it writes "John Jr." into the new allocation.
    // Third, it frees the original heap memory. name_to_extend now points to deallocated memory.
    //
    // At L4, the frame for add_suffix is gone. This function returned name_to_extend,
    // transferring ownership of the string to full_name.

    // Note that variables can not be used after they have been moved.
    // println!("name is {name}"); // this would panic, because name is no longer valid.

    // Moved heap data principle: if a variable x moves ownership of heap data to
    // another variable y, then x cannot be used after the move.

    // You can avoid moving heap data by cloning it.
    // let name_clone = name.clone(); // this creates a new heap memory for the string "John"
    // and copies the data into it.
    // println!("name is {name}, name_clone is {name_clone}"); // this would print "name is John, name_clone is John"

    let name = String::from("John"); 
    let name_clone = name.clone();
    let full_name = add_suffix(name_clone);
    // name_clone wouldn't be valid anymore, because it was moved to full_name.
    println!("name is {name}, full_name is {full_name}"); 

}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut name_to_extend: String) -> String {
    name_to_extend.push_str(" Jr.");
    name_to_extend
}