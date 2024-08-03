fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }; //updating some fields but basicalling copying user1 to user2
    //OR (we are MOVING data, user1 is invalid now for the fields which were copied only)
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };//any remaining fields should get their values from the corresponding fields in user1

}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}