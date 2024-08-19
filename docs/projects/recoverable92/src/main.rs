use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    //tye of File::open is a Result<T, E>

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        // Err(error) => panic!("GOT ERRRRR: {error:?}")
        // OR
        Err(e) => match error.kind() { //kind() specific to ::ErrorKind lib
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem generatignt he fiel: {e:?}")
            },
            other_error => { //catches all (?)
                panic!("Problem opening file: {other_error:?}");
            }
        }
    };
}
// // OR
fn main() {
    let greeting_file_result = File::open("hello.txt").unwrap(); //if err, panic occurs
    // OR
    let greeting_file_result = File::open("hello.txt").expect("no file sorzz"); //lets chosoe err message
}

// // PROPOGATING
// Long method
fn read_username_frm_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let username_file = match username_file_result{
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut String){ //read to string alo returns a result noice
        Ok(_) => username,
        Err(e) => Err(e)
    }
}

// OR

// Short method
fn read_username_frm_file() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)//read_to_string ka Ok doesnt have username in it, so returned separately
}

// OR

// chaint he functionssss noice
fn read_username_frm_file() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// OR

// use fs function

fn read_username_frm_file() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

// Cool usade of ?
fn get_last_char(text: &str) -> Option<char> {
    text //takes text arg
        .lines() //lines iteration
        .next()? //checks if line exist else none
        .chars() //char iteration
        .last() //last char
}