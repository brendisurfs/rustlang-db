// PROJECT - kvstore: store key value pairs in a file format of our choice.
use std::collections::HashMap;

fn main() {
    // arguments to insert into the  CLI app. 
    // args is an itterator.
    let mut arguments = std::env::args().skip(1);
    // on linux, the first arg is the path to the application. we dont want that here. we want our stuff to be optional.
    let key: String = arguments.next().expect("key not found, please add a key.");
    let value: String = arguments.next().expect("value not found, please add a value.");
    println!("The key is {}, the value is {}", key, value);
    
    // - create our kv pair
    let contents = format!("{} = {}\n", key, value); 
    
    // - write out to file.
    let write_result = std::fs::write("kv.db", contents).expect("something didnt go right in std::fs::write");
    let database = Database::new().expect("database::new() crashed");
}

// --- Database Struct and Impl
struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> { 

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, "=");
            let key = chunks.next().expect("no key");
            let value = chunks.next().expect("no value.");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: map
        }) 
    }
}

    // *** THIS IS WHAT THE QUESTION MARK EXTENDS TO KINDA ***
    //     // read the kv.db file
    //    let contents = match std::fs::read_to_string( "kv.db") {
    //         Ok(c) => c, 
    //         Err(error) => {
    //             return Err(error);
    //         }
    //     }; 
