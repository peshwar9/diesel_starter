use diesel_demo::*;
use diesel_demo::models::*;
use diesel::prelude::*;

use std::io::{Read, stdin};


fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");

    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len()-1)];
    println!("\n OK let's write {}. Press {} when finished\n", title, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, &title, &body);
    println!("\n saved post {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";