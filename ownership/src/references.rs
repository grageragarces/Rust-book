// // REFERENCES: an address we can follow to access the data stored at that address (~pointer), guaranteed to point to a valid value of a particular type for the life of that reference (!pointer), immutable
// // &, opp (dereferencing) *
// fn main() {
//     let s1 = String::from("hello"); //from: string literal to string

//     let len = calculate_length(&s1); //& is the referencing (opp *)

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because s does not have ownership of what
//   // it refers to, the String is not dropped.


// // MUTABLE REFERENCES: &mut
// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s); //can only have one &mut at a time (prevents data races at compile time)
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// // SAFE FROM DANGLING
// fn main() {
//     // let reference_to_nothing = dangle();
//     let reference_to_something = no_dangle();
// }

// // fn dangle() -> &String { // dangle returns a reference to a String

// //     let s = String::from("hello"); // s is a new String

// //     &s // we return a reference to the String, s
// // } // Here, s goes out of scope and is dropped, so its memory goes away.
// //   // Danger!

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s //this is being returned btw
// }