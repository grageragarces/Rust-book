//We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}


// FUN FACT

// exception of function that compiles without reference lifetime (historical)
// predictable and followed a few deterministic patterns
// mantained cause it’s possible that more deterministic patterns will emerge and be added to the compiler
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
