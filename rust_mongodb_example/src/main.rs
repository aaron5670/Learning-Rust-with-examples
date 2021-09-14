use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};
use std::error::Error;
use tokio;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let client_options = ClientOptions::parse("mongodb://root:s3cr3t_pass@localhost:27017/?authSource=admin").await?;

    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    println!("\nDatabases:");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("next-tailwind");

    // List the names of the collections in that database.
    println!("\nCollection names:");
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    let collection = db.collection::<Document>("users");

    let docs = vec![
        doc! {
            "name" : "Aaron Doe",
            "title" : "Director Technician",
            "department" : "Happy Programmer",
            "role" : "Super Admin",
            "email" : "aaron.doe@example.com"
        }
    ];

    // Insert some documents into the "next-tailwind.users" collection.
    collection.insert_many(docs, None).await?;
    println!("\nDocuments inserted into the next-tailwind.users collection");

    // Keep terminal open on end
    println!("Press any key to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
