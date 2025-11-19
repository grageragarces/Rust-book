
fn main() {
    let v = vec![1, 2, 3, 4];

    // filter to keep even number & *2
    let a: Vec<_> = v.iter().filter(|x: &&i32| *x % 2 == 0).map(|x: &i32| x * 2).collect();

    // doubles every number & keep even
    let b: Vec<_> = v.iter().map(|x: &i32| x * 2).filter(|x: &i32| x % 2 == 0).collect();

    println!("{} {}", a[0], b[0]);
}