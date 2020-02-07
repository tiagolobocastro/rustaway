use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => {
            if s.len() > 0 {
                Ok(s)
            } else {
                let e = Error::new(ErrorKind::Other, "no username found");
                Err(e)
            }
        },
        Err(e) => Err(e),
    }
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3i() -> Option<String> {
    //Some("tiago".to_string())
    None
}
fn read_username_from_file3() -> Option<String> {
    let o = read_username_from_file3i()?;
    Some(o)
}

fn main() {
    println!("Hello, world!");

    let _r = read_username_from_file();
    let _r = read_username_from_file2();
    let _r = read_username_from_file1();

    match _r {
        Ok(s) => println!("username: {}", s),
        Err(e) => println!("error: {}", e),
    }
    
    let _r = read_username_from_file3();
    
    match _r {
        Some(s) => println!("username: {}", s),
        None => println!("error: nothing"),
    }

    let _x = Guess::new(10);
    let _y = _x.value();
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess::from(value)
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    fn from(val: i32) -> Guess {
        Guess { value: val }
    }
}
