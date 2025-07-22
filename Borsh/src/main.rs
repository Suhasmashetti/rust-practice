use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize};
struct User {
    username: String,
    password: String,
}

fn main() {
    let u = User {
        username: String::from("suhas"),
        password: String::from("suhas123"),
    };

    let mut v: Vec<u8> = Vec::new();
    let ans = u.serialize(&mut v);
    match ans {
        Ok(_) => {
            println!("Serialization successful: {:?}", v);
        },
        Err(e) => {
            println!("Serialization failed: {}", e);
        }
    }

    match User::try_from_slice(&v) {
        Ok(user) => println!("Deserialized user: {:?}", user),
        Err(e) => println!("Deserialization failed: {}", e),
    }
}
