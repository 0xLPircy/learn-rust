use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    //tye of File::open is a Result<T, E>

    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => panic!("GOT ERRRRR: {error:?}")
    };
}
