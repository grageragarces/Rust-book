// STRUCTs are similar to tuples
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User { // function that returns a User instance
    User {
        active: true,
        username: username,
        // ability to comment on next line = the field init shorthand
        email, //: email,
        sign_in_count: 1,
    }
}


fn main() {

    let user1 = User { //instance of struct
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // . notation to access/change something
    user1.email = String::from("anotheremail@example.com");

    
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // Equiv to:

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
