use diesel_demo::*;
use diesel_demo::models::*;
use diesel::prelude::*;

use std::io::{Read, stdin};
use std::env::args;

fn main() {
    use schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected id")
    .parse::<i32>().expect("Invalid number");

    let connection = establish_connection();

    let post_deleted = diesel::delete(posts.find(target))
        .execute(&connection)
        .expect("Error in deleting post");

    println!("Deleted post {} ", post_deleted);
}

