use diesel_demo::*;
use diesel_demo::models::*;
use diesel::prelude::*;

fn main() {
    use schema::posts::dsl::*;

    let connection = diesel_demo::establish_connection();

    let results = posts
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
        println!("{}",post.title);
        println!("-----\n");
        println!("{}",post.body);
    } 
}
