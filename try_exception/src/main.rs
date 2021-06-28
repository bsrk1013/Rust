use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let result = read_file("hello.txt");
    match result{
        Ok(s) => println!("result:{}", s),
        Err(e) => println!("err:{:?}", e),
    }

    let result2 = read_file_shortcut("hello2.txt").expect("not found hello2 file...");
    println!("result2:{}", result2);
}

fn read_file(name: &str) -> Result<String, io::Error>{
    let file = File::open(name);
    let mut file = match file {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    let mut result = String::new();
    match file.read_to_string(&mut result){
        Ok(_) => Ok(result),
        Err(e) => Err(e),
    }
}

fn read_file_shortcut(name: &str) -> Result<String, io::Error>{
    let mut file = File::open(name)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}