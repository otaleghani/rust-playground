// a classical struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs. The values are not named, and can be deconstructed
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("some@email.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("someotheruser123"),
        email: String::from("some@other.com"),
        sign_in_count: 2,
    };

    let mut user3 = build_user(String::from("giordano"), String::from("bruno"));

    println!("user 1 {0}", user1.email);

    user2.active = false;
    println!("user 2 {0}", user2.active);

    user3.active = false;
    println!("user 3 {0} {1} {2}", user3.active, user3.username, user3.email);

    let user4 = User {
        active: true,
        ..user3     // Update syntaxt give you the ability to change just the
                    // fields that you want to update from a given user
                    // KEEP in mind, after this user3 will not be available in this 
                    // scope, reason why is that we have 2 String type that 
                    // where moved from user3 to user4. We will need to create
                    // new String(s) to create an hard copy
    };
    println!("user 3 {0} {1} {2} {3}", user4.active, user4.username, user4.email, user4.sign_in_count);

    let color1 = Color(0,1,2);
    println!("{0}", color1.2);

    // let tuple = (1,2,3);
    // let (a, b, c) = tuple;

    let point1 = Point(1,2,3);
    let Point(x, y, z) = point1;
    println!("{x}, {y}, {z}");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,   // You can omit the field name if these are equal
        email,      // This is called "field init shorthand"
        sign_in_count: 1,
    }
}
