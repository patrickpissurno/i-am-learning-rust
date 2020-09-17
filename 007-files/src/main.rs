use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // more verbose way
    {
        let mut f = match File::open("hello.txt"){
            Ok(f) => f,
            Err(_) => panic!("Error opening the file")
        };

        let mut txt = String::new();
        match f.read_to_string(&mut txt){
            Ok(_) => (),
            Err(_) => panic!("Failed to read file contents")
        };

        println!("File contents: '{}'", txt);
    }

    // is the same as
    {
        let mut f = File::open("hello.txt").expect("Error opening the file");
        let mut txt = String::new();
        f.read_to_string(&mut txt).expect("Failed to read file contents");

        println!("File contents: '{}'", txt);
    }

    // function call (verbose version)
    {
        let txt = read_file_and_propagate_errors_verbose("hello.txt").expect("Failed to read file via function call");
        println!("File contents: '{}'", txt);
    }

    // function call (more concise version)
    {
        let txt = read_file_and_propagate_errors_concise("hello.txt").expect("Failed to read file via function call");
        println!("File contents: '{}'", txt);
    }

    // function call (with internal chaining)
    {
        let txt = read_file_and_propagate_errors_chained("hello.txt").expect("Failed to read file via function call");
        println!("File contents: '{}'", txt);
    }

    // built-in function that does the same
    {
        let txt = fs::read_to_string("hello.txt").expect("Failed to read file via built-in function call");
        println!("File contents: '{}'", txt);
    }
}

// more verbose way
fn read_file_and_propagate_errors_verbose(fname: &str) -> Result<String, io::Error>{
    let mut f = match File::open(fname){
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut txt = String::new();
    match f.read_to_string(&mut txt){
        Ok(_) => (),
        Err(e) => return Err(e)
    };

    Ok(txt)
}

// more concise way
fn read_file_and_propagate_errors_concise(fname: &str) -> Result<String, io::Error>{
    let mut f = File::open(fname)?;
    let mut txt = String::new();

    f.read_to_string(&mut txt)?;

    Ok(txt)
}

// chained way
fn read_file_and_propagate_errors_chained(fname: &str) -> Result<String, io::Error>{
    let mut txt = String::new();

    File::open(fname)?.read_to_string(&mut txt)?;

    Ok(txt)
}