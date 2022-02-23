use std::fmt::Debug;

#[derive(Debug,PartialEq)]
enum FileState{
    Open,
    Closed
}

#[derive(Debug)]
struct File{
    // Fields are private by default and but can be accessed within the module that
    // defines them
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File{
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
}
