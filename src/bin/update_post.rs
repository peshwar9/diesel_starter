use diesel_demo::*;
use diesel_demo::models::*;
use diesel::prelude::*;

use std::io::{Read, stdin};
use std::env::args;

fn main() {
    use schema::posts::dsl::*;

    let post_id = 
        args()
        .nth(1)
        .expect("update post requires a post id")
        .parse::<i32>()
        .expect("Invalid Id");

    println!("What is updated text you want ?");

    let mut updated_body = String::new();
    stdin().read_line(&mut updated_body).unwrap();
    let connection = establish_connection();

    let post = diesel::update(posts.find(post_id))
        .set(body.eq(updated_body))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post id {}",post_id));
    println!("Updated post {} successfully",post_id);
    
}