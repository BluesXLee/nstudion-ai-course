// src/ch10_lifetimes.rs

pub fn run_exercises() {
    println!("--- Chapter 10: Lifetimes & Generics ---");
    
    println!("\n=== Part 1: Generics ===");
    generics_intro();
    generics_functions();
    generics_structs_enums();

    println!("\n=== Part 2: Lifetimes ===");
    lifetimes_intro();
    lifetimes_concepts();
    lifetimes_examples();
    lifetimes_elision_examples();
    lifetimes_static_examples();
}

fn generics_intro() {
    println!("\n[1] Generics Introduction");
    println!("Generics allow us to write code that can work with multiple data types.");
    println!("Instead of writing the same function for i32, f64, and String, we use a type parameter.");
}

fn generics_functions() {
    println!("\n[2] Functions with Generics");
    
    fn print_anything<T>(item: T) 
    where T: std::fmt::Display 
    {
        println!("Value: {}", item);
    }

    println!("Printing an integer: ");
    print_anything(42);

    println!("Printing a string: ");
    print_anything("Hello Rust!");
}

fn generics_structs_enums() {
    println!("\n[3] Generics with Structs and Enums");

    struct Container<T> {
        value: T,
    }

    enum Optional<T> {
        Some(T),
        None,
    }

    let int_container = Container { value: 10 };
    let str_container = Container { value: "Hello" };

    let some_val = Optional::Some(5.5);
    let none_val: Optional<i32> = Optional::None;

    println!("Container (int): {}", int_container.value);
    println!("Container (str): {}", str_container.value);

    match some_val {
        Optional::Some(v) => println!("Optional value: {}", v),
        Optional::None => println!("No value"),
    }

    match none_val {
        Optional::Some(v) => println!("Optional value: {}", v),
        Optional::None => println!("No value"),
    }
}

fn lifetimes_intro() {
    println!("\n[1] What are Lifetimes?");
    println!("Lifetimes are a way for the Rust compiler to ensure that all references are valid.");
    println!("The 'Borrow Checker' uses lifetimes to prevent 'dangling references' (references to memory that has been freed).");
}

fn lifetimes_concepts() {
    println!("\n[2] Lifetime Annotations");
    println!("Lifetime annotations (like 'a) describe the relationship between the lifetimes of different references.");
    println!("They don't change how long a value lives; they just tell the compiler how they relate.");
    
    println!("\nExample: The 'Longest' function");
    println!("If we have a function that takes two references and returns one, Rust needs to know:");
    println!("'Which input reference is the return value tied to?'");
}

fn lifetimes_examples() {
    println!("\n=== [3] Lifetime Examples ===");

    // Example 1: The 'Longest' function
    // This function takes two string slices and returns the longest one.
    // We use 'a to tell the compiler that the returned reference will live 
    // as long as the SHORTEST of the two input lifetimes.
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = "abcd"; // 'static lifetime
    let string2 = "efgh"; // 'static lifetime

    let result = longest(string1, string2);
    println!("The longest string is: {}", result);

    println!("\n--- Moving to Example 2: Structs with Lifetimes ---");

    // Example 2: Structs with Lifetimes
    // If a struct holds a reference, it MUST have a lifetime annotation.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    {
        let text = String::from("This is an important excerpt.");
        let excerpt = ImportantExcerpt {
            part: &text,
        };
        println!("Excerpt: {}", excerpt.part);
    }
}

fn lifetimes_elision_examples() {
    println!("\n=== [4] Lifetime Elision ===");
    println!("Elision is Rust's ability to automatically infer lifetimes in many common cases.");
    println!("You don't always need to write 'a if the compiler can figure it out.");

    // Example 1: Function with one input reference
    // The compiler assumes the output lifetime is the same as the input lifetime.
    fn first_param_elision(s: &str) -> &str {
        s
    }

    let input = "Hello Elision";
    let result = first_param_elision(input);
    println!("Elided result: {}", result);

    println!("\nExample 2: Multiple input references (The compiler needs help!)");
    println!("If there are multiple input references, the compiler can't always know which one to use.");
    println!("In such cases, we MUST use explicit annotations.");
}

fn lifetimes_static_examples() {
    println!("\n=== [5] The 'static Lifetime ===");
    println!("'static is a special lifetime that lasts for the entire duration of the program.");
    println!("It is often used for string literals or global constants.");

    // Example 1: String Literals
    // String literals have a 'static lifetime because they are embedded in the binary.
    let s: &'static str = "I am static";
    println!("Static string: {}", s);

    // Example 2: Using 'static in a function
    fn print_static(s: &'static str) {
        println!("Static: {}", s);
    }

    print_static(s);
}
