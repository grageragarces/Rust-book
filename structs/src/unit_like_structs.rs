struct AlwaysEqual; //empty struct

fn unit() { //can then be used
    let subject = AlwaysEqual;
}

//can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself