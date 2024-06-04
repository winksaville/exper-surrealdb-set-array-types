use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct MyRecord {
    array_field: Vec<f64>,
    set_field: Vec<f64>,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Connect to SurrealDB
    let db = Surreal::new::<Mem>(()).await?;
    println!("Created Mem db");

    // Select the namespace and database
    db.use_ns("test_namespace").use_db("test_database").await?;
    println!("Add ns and db to use");

    // Define the table to have schema and define field for the table
    db.query("DEFINE TABLE my_table SCHEMAFULL").await?;
    db.query("DEFINE FIELD array_field ON TABLE my_table TYPE array<float>")
        .await?;
    db.query("DEFINE FIELD set_field ON TABLE my_table TYPE set<float>")
        .await?;
    println!("Defined table and field");

    // Create a record with an array of floats with duplicated values
    let array_of_data = vec![1.1, 1.1, 2.2, 2.2, 3.3, 3.3, 4.4, 4.4, 5.5, 5.5];
    let my_record = MyRecord {
        array_field: array_of_data.clone(),
        set_field: array_of_data.clone(),
    };

    // Create the data base with the record
    let data: Vec<MyRecord> = db.create("my_table").content(my_record).await?;
    println!("Create table my_table with a record");

    // Print the created record
    println!("data: {:?}", data);

    // Assert the created record has expected lengths and contents
    assert_eq!(data[0].array_field.len(), 10);
    assert_eq!(data[0].array_field, array_of_data);
    assert_eq!(data[0].set_field.len(), 5);
    assert_eq!(data[0].set_field, vec![1.1, 2.2, 3.3, 4.4, 5.5]);

    Ok(())
}
