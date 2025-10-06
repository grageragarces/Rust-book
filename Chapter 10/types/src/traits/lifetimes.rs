
fn main_orig() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// } //-> ERROR: generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y
// // we don’t know whether the if case or the else case will execute
// // we also don’t know the concrete lifetimes of the references that will be passed in


// //Lifetime Annotation Syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // for some lifetime 'a, the function takes two parameters
    // both are string slices that live at least as long as lifetime 'a
    // function signature tells Rust that the string slice returned from the function will live at least as long as lifetime 'a
    // =>  lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");
}
