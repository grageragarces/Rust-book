enum Option<T> { //defined in the standard library
    None,
    Some(T),
}

let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

// NOTE: you have to convert an Option<T> to a T before you can perform T operations with it