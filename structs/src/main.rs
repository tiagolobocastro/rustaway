
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

mod test;

fn main() {
    let user = User {
        username: String::from("socio"),
        email: String::from("a@a.com"),
        sign_in_count: 0,
        active: false,
    };

    let user2 = User {
        username: String::from("socio2"),
        ..user
    };

    println!("{:#?}", user2);
   
    stuff();
    stuff();
    stuff();
    stuff();

    test::stuff2();
    test::stuff2();
    test::stuff2();
}

fn stuff() {
    println!("test123...");
}