
fn main() {
    // arguments to insert into the  CLI app. 
    // args is an itterator.
    let mut arguments = std::env::args().skip(1);

    // on linux, the first arg is the path to the application. 
    // we dont want that here.
    
    // we want our stuff to be optional.
    let key: String = arguments.next().unwrap();
    let value: String = arguments.next().unwrap();
    println!("The key is {}, the value is {}", key, value);

}
