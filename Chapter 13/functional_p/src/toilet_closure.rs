// f causes s to be immediately dropped
fn main() {
    let f = |_| (); // sometimes called the "toilet closure"
    let s = String::from("Hello");
    f(s);
}