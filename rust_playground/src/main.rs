fn main() {
    // 1.a Simple Greeting Example
    let name: &str = "justin";
    let greeting_statement = format!("Hello, greetings from Rust to: {}", name);
    // We could achieve the same with: println!("{}", greeting_statement);
    println!("{greeting_statement}");

    // 1.b Functional Greeting Example 
    let name: &str = "thy";
    let msg = greeting_func(name);
    println!("{msg}");
}

// Greeting Function 
fn greeting_func(name: &str) -> String {
    let base_statement = format!("Greeting in Rust via functions to: {name}");
    base_statement
}
