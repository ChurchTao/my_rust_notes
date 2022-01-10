use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    // test
    // let s = match read_from_file("hello.txt") {
    //     Ok(s) => s,
    //     Err(e) => panic!("file not found"),
    // };

    // test2
    let s = read_from_file2("hello.txt").unwrap();
    println!("{}", &s);
}

fn read_from_file(name: &str) -> Result<String, std::io::Error> {
    let file = File::open(name);
    let mut f = match file {
        Ok(f) => f,
        Err(e) => {
            let mut f = File::create(name).unwrap();
            f.write("hello world!".as_bytes()).unwrap();
            return Err(e);
        }
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file2(name: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(name)?.read_to_string(&mut s)?;
    Ok(s)
}
