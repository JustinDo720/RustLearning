# Starting Rust 

## Downloading 

We download rust [here](https://rust-lang.org/tools/install/)

& VSCode build tools [here](https://visualstudio.microsoft.com/downloads/)
- Scroll Down 
- Tools for VS Studio 
- Build tools --> for Complier 

## Startup 

We need the most stable version 
- `rustup install stable` + `rustup default stable`

Creating a Project 
- `cargo new project_name` 
- `cd project_name` 
- `cargo run`

## Understanding Project 

**rust_playground/src/main.rs**

```rust
// fn is similar to our def to create a function
fn main() {
    // printIn! is a macro NOT a function
    println!("Hello, world!");
}
```
1) Everything runs at `main()` which is similar to `if __name__ == '__main__'`
2) We have a println! (print line marco) which is simlar to our print statement 

We would go ahead and `cargo run` in order to run our actual script 
1) alternatively you could `rustc` to complie

## Why Rust 

Python is limited by GIL --> uses one core to execute bytecode and run our program via an interpreter 
1) If we want to use multiple cores you would need to use `multiprocessing` 
2) Even then you'll have to manage the communication between the cores 
3) which is ineffecient 

## Macros 

We saw `println()` which we could actually do some formatting: 
1) `println!("Hello, {}!", "justin");` --> replaces the `{}` with w.e that comes afterwards
2) `let greeting = format!("hello {}", name)`

```rust
fn main() {
    let name: &str = "justin";
    let greeting_statement = format!("Hello, greetings from Rust to: {}", name);
    // We could achieve the same with: println!("{}", greeting_statement);
    println!("{greeting_statement}");
}
```
1) We set a name variable with a `name: &str` which means it is **immutable** string 
2) `format!()` marco which we then use with `println!("{}", greeting_statement)`

## Functions + Arguments 

```rust 
fn main(){
    let name: &str = "Thythy";
    let msg = greeting_function(name);

    println!(msg);

}

fn greeting_function(name: &str) -> String {
    format!("Greeting via function for Rust to: {name}")
}
```
1) Notice we need a `;` after every line 
2) Additionally we use type hints for our function (&str for immuatable string then we return a String)
3) To return a string (Idiomatic Rust return) it's the last line of the function with NO semicolon 

## Borrowers vs Owners (80% of Rust)

Rust does not have a **Garbage Collection** but you also don't have to **manually** free up memory that's why we have this:
1) Borrower vs Owner System 
2) Owner = Responsible for Memory 
3) Borrower = Temporary Access 

**Ownership**

---

`let string_owner_heap = String::from("Owning string within the heap.")`
1) `String::from()` let's the object own memory within the heap 

Just remember this:

> Accept `&str` and Store `String`

---

```rust
fn main() {
    let mut string_owner_heap = String::from("Owning String within heap");
    string_owner_heap.push_str(" Adding more to the heap");

    println!("{string_owner_heap}");
}
```
1) using `push_str` to a mutable will allow us to ADD more strings to the existing string within our heap

**Borrowing**

```rust
fn main() {
    // Borrowing from owner: Immutable
    let string_borrower = &string_owner_in_heap;
    println!("Borrower: {string_borrower}");
}
```
1) Borrowed with the `&string_owner_in_heap`
2) Ownership stays with the `string_owner_heap` 
3) No mutations with our borrowed 

```rust
fn main() {
    // Borrowing from owner: Mutable 
    let string_borrower_mutable = &mut string_owner_in_heap;
    string_borrower_mutable.push_str(" Pushing through borrowed mutable.");
    println!("Borrower: {string_borrower}");

    // Borrowing from owner: Immutable
    let string_borrower = &string_owner_in_heap;
    println!("Borrower: {string_borrower}");
}
```
1) You can only have **ONE** mutable borrower 
2) This must come BEFORE any immutable borrwers 
3) Changing this will change the Owner 
4) Immuatable will reflect all the changes which is why mutable must come first 

---

**Timing and Existence**

**Immutables** can last forever you could consume / read how many times you want. **Mutables** can only exist 1 at a time

```rust
fn main() {
    let mut s = String::from("Hello");

    // First AND ONLY mut borrower 
    let mut_borrower = &mut s;
    mut_borrower.push_str(" New String to Memory");
    
    // YOU CANNOT HAVE ANOTHER MUT BORROWER
    let second_borrower = &mut s;

}
```

## Project Structure (Multiple Mains)

Instead of running one `main.rs` with `cargo run` you could have `src/bin` and have a bunch of `.rs` files 
1) then we could run a specific file if we have a file name: `bin/ownership.rs`
2) `cargo run --bin ownership`

