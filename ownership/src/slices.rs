
// small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string
// no space -> whole string = 1 word

pub fn run_problem() { //need to make the function public
    let mut s = String::from("hello world");

    let word = first_word_with_slice(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn def_slice() { //SLICES: this is how they look like
    let s = String::from("hello world");

    let hello = &s[..5]; // bits 0 - 5
    let world = &s[6..11];

}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //look for first space
            return &s[0..i]; //return the index
        }
    }

    &s[..]
}

pub fn main_slice() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word_with_slice(&my_string[0..6]); 
    let word = first_word_with_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s.
    let word = first_word_with_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole.
    let word = first_word_with_slice(&my_string_literal[0..6]);
    let word = first_word_with_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_with_slice(my_string_literal);
}