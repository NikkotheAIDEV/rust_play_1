use std::fs::OpenOptions;
use std::io::Write;

pub fn open_file_and_write(path: String) -> Result<(), std::io::Error> {
    // As far as I'm aware, the String type should do a heap allocation under the hood, but
    // putting it inside a Box<T>(heap allocation) makes me feel smarter, so why not. However
    // just because you can, doesn't mean you should. But I will do it anyways...
    let mut data: Box<String> = Box::new(String::new());

    // OpenOptions are something of a 'configuration' on how a file should be open, read and written to.
    let mut file_handle = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    println!("Enter thing to write in file: ");

    // getting the user's input and writing it to the data.
    let _ = std::io::stdin().read_line(&mut data)?;

    // writing user's input to a file.
    let _ = writeln!(&mut file_handle, "{}", data.as_ref())?;
    Ok(())
}