fn main() {
    // Sets ownership within the heap and let's the object own that memory 
    let mut string_owner_in_heap = String::from("Hello in heap");
    string_owner_in_heap.push_str(" Additional String to Heap");

    // Borrowing from owner: Mutable 
    let string_borrower_mutable = &mut string_owner_in_heap;
    string_borrower_mutable.push_str(" Pushing through borrowed mutable.");

    // Borrowing from owner: Immutable
    let string_borrower = &string_owner_in_heap;


    println!("Owner: {string_owner_in_heap}");
    println!("Borrower: {string_borrower}");
}