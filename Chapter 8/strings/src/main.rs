fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

}

fn hello() { // all valid
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn concatenate(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

}

// RUST STRINGS DON'T SUPPORT INDEXING!
// will break:
fn broken_call(){
    let s1 = String::from("hi");
    let h = s1[0];}
// A String is a wrapper over a Vec<u8>
// &hello[0] == 104 (the UTF-8 first byte, not h)
// for non english hello it's even worse it may require two bits per letter 
// let hello = "Здравствуйте"; З = 208,151

// But if we want to we can actually print the byte/chars:
fn print_char(){
    for c in "Зд".chars() {
        println!("{c}");
    }
}
fn print_byte(){
    for c in "Зд".chars() {
        println!("{c}");
    }
}