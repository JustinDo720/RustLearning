fn main() {
    let x = find();
    
    match x {
        Some(v) => println!("value: {}", v),
        None => println!("No value")
    }

    let n = Some(32);
    let doubled_n = n.map(|v| v*2);

    match (n, doubled_n) {
        (Some(original), Some(doubled)) => {
            println!("Doubled the value of {original} is: {doubled}")
        }
        _ => {
            println!("No values to double")
        }
    }
}

fn find() -> Option<i32> {
    // I32 is an integer ... We're using Option to replace None / null
    Some(42)
}