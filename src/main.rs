// By convention the line below is used to annotate the current module
// It kind a description of the crate and appears at the beginning of documentation.
//! Simulate files one step at a time
use std::fmt::Debug;
use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
enum FileState{
    Open,
    Closed
}

/// Represents a file,
/// which probably lives at file system.
#[derive(Debug)]
pub struct File{
    // Fields are private by default and but can be accessed within the module that
    // defines them
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FileState::Closed => write!(f, "CLOSED"),
            &FileState::Open => write!(f, "OPEN"),
        }
    }
}

impl Display for File{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"f<{},({})>", self.name, self.state)
    }
}

impl File{
    /// New files are assumed to be empty, but a name is required.
    // new is convention name to create a struct instance or object creation
    fn new(name: &str)-> File{
        File { name: String::from(name), data: Vec::new(), state: FileState::Closed }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize,String> /* `String is provided for arbitrary error messages` */ {
        if self.state!=FileState::Open{
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
    /// Returns file's length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Returns the file's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }
}


fn open(mut f: File)-> Result<File,String>{
    f.state= FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File,String>{
    f.state = FileState::Closed;
    Ok(f)
}



fn main() {
    let mut f5 = File::new("th5.txt");
    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err(){
        print!("Error checking is working");
    }

    f5 = open(f5).unwrap(); // `unwrap()` shouldn't be used in production code
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    // Convert a byte array to a UTF8 string, invalid characters will be showed as  �
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}",f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);

    let f1_name = f5.name();
    let f1_length = f5.len();

    // Use the local implementation of `Display` trait
    println!("{}",f5);
    println!("{} is {} bytes long",f1_name,f1_length);
}
