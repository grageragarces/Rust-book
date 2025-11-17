
fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}

// The first time we call example_closure with the String value, 
// the compiler infers the type of x and the return type of the closure to be String. 
// Those types are then locked into the closure in example_closure, 
// and we get a type error when we next try to use a different type with the same closure.