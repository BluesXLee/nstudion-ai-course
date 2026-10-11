// src/ch11_smart_pointers.rs

pub fn run_exercises() {
    println!("--- Chapter 11: Smart Pointers ---");
    
    println!("\n=== Part 1: Introduction ===");
    smart_pointers_intro();

    println!("\n=== Part 2: Box<T> ===");
    box_examples();

    println!("\n=== Part 3: Rc<T> ===");
    rc_examples();

    println!("\n=== Part 4: RefCell<T> ===");
    refcell_examples();
}

fn smart_pointers_intro() {
    println!("\n[1] What are Smart Pointers?");
    println!("In Rust, a 'smart pointer' is a data structure that acts like a pointer but has additional metadata and capabilities.");
    println!("Unlike regular references, smart pointers own the data they point to (or share ownership) and can manage resource cleanup.");
s
    println!("Common smart pointers include: Box<T>, Rc<T>, RefCell<T>, and Arc<T>.");
}

fn box_examples() {
    println!("\n[1] Box<T> - Heap Allocation");
    println!("Box<T> is the simplest smart pointer. It allows you to store data on the heap rather than the stack.");
    println!("This is useful when you have a large type, or when you want to own a value whose size is unknown at compile time.");

    // Example: Storing a large value on the heap
    struct LargeData {
        data: [u8; 1024],
    }

    let boxed_data = Box::new(LargeData {
        data: [0; 1024],
    });

    println!("Boxed data is allocated on the heap.");
    println!("The size of the box itself is just the size of a pointer.");
    println!("Accessing data: first byte is {}", boxed_data.data[0]);
}

fn rc_examples() {
    println!("\n[1] Rc<T> - Reference Counting");
    println!("Rc<T> stands for 'Reference Counted'. It allows multiple owners of the same data.");
    println!("It is used when you need to share data within a single thread.");
    println!("The data is only cleaned up when the last 'Rc' pointer is dropped.");

    use std::rc::Rc;

    let shared_data = Rc::new(String::from("Shared ownership content"));

    {
        let user_a = Rc::clone(&shared_data);
        println!("User A sees: {}", user_a);
        println!("Reference count: {}", Rc::strong_count(&shared_data));

        {
            let user_b = Rc::clone(&shared_data);
            println!("User B sees: {}", user_b);
            println!("Reference count: {}", Rc::strong_count(&shared_data));
        }
        
        println!("User B left. Reference count: {}", Rc::strong_count(&shared_data));
    }

    println!("User A left. Reference count: {}", Rc::strong_count(&shared_data));
}

fn refcell_examples() {
    println!("\n[1] RefCell<T> - Interior Mutability");
    println!("RefCell<T> allows you to mutate data even when you have an immutable reference to it.");
    println!("This is known as 'Interior Mutability'. It moves the borrowing rules from compile-time to runtime.");
    println!("Note: This can cause panics if you violate borrowing rules at runtime!");

    use std::cell::RefCell;

    // Example: Mutating a value through an immutable reference
    let data = RefCell::new(vec![1, 2, 3]);

    // We can borrow_mut even though 'data' is not declared as 'mut'
    // (though in a real scenario, the variable itself might be immutable)
    {
        let mut mutable_borrow = data.borrow_mut();
        mutable_borrow.push(4);
        println!("Mutated vector: {:?}", mutable_borrow);
    }

    {
        let immutable_borrow = data.borrow();
        println!("Read vector: {:?}", *immutable_borrow);
    }
    
    println!("Final vector: {:?}", data.borrow());
}
