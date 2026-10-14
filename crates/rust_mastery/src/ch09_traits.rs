// src/ch09_traits.rs
// Chapter 9: Traits

pub fn run_exercises() {
    println!("--- Chapter 9: Traits ---");
    
    // 1. Basic Trait Implementation
    println!("\n[1] Basic Trait Implementation");
    let dog = Dog {};
    let cat = Cat {};
    
    make_sound(&dog);
    make_sound(&cat);

    // 2. Trait Bounds
    println!("\n[2] Trait Bounds");
    print_info(&dog);
    print_info(&cat);

    // 3. Default Implementations
    println!("\n[3] Default Implementations");
    run_default_impls();

    // 4. Dynamic Dispatch (Trait Objects)
    println!("\n[4] Dynamic Dispatch (Trait Objects)");
    run_dynamic_dispatch();
}

// --- Section 1 & 2: Basics ---

trait Speak {
    fn say_hello(&self) -> String;
}

struct Dog {}
impl Speak for Dog {
    fn say_hello(&self) -> String {
        "Woof!".to_string()
    }
}

struct Cat {}
impl Speak for Cat {
    fn say_hello(&self) -> String {
        "Meow!".to_string()
    }
}

fn make_sound(animal: &impl Speak) {
    println!("The animal says: {}", animal.say_hello());
}

fn print_info<T: Speak>(item: &T) {
    println!("Info: {}", item.say_hello());
}

// --- Section 3: Default Implementations ---

trait Greet {
    fn greeting(&self) -> String {
        "Hello!".to_string() // Default implementation
    }
}

struct Human {}
impl Greet for Human {} // Uses default

struct Robot {}
impl Greet for Robot {
    fn greeting(&self) -> String {
        "Beep Boop!".to_string() // Overrides default
    }
}

fn run_default_impls() {
    let h = Human {};
    let r = Robot {};
    println!("Human says: {}", h.greeting());
    println!("Robot says: {}", r.greeting());
}

// --- Section 4: Dynamic Dispatch ---

trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

struct Square { side: f64 }
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

fn run_dynamic_dispatch() {
    // We use a Vec of Trait Objects (&dyn Shape)
    // This allows us to store different types in the same collection
    // as long as they all implement the same trait.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
    ];

    for shape in shapes {
        println!("Shape area: {}", shape.area());
    }
}
