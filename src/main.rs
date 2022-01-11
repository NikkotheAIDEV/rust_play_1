use std::fs::{OpenOptions};
use std::io::{Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {

    open_file_and_write("file.txt".to_string()).unwrap();
}

fn open_file_and_write(path: String) -> Result<(), std::io::Error> {
    // let mut data = Box::new(String::new());
    let  data: Arc<Mutex<Box<String>>> = Arc::new(Mutex::new(Box::new(String::new()))); //Box::new(String::new());
    //let mut data = String::new();
    let mut file_handle = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    let mutex_data_for_thread = data.clone();//Arc::clone(&data);

    println!("Enter thing to write in file: ");
    let thread_handle: JoinHandle<Result<(), std::io::Error>> = thread::spawn( move || {
        let _ = std::io::stdin().read_line(&mut mutex_data_for_thread.lock().unwrap())?; // &mut data
        let _ = writeln!(&mut file_handle, "{}", mutex_data_for_thread.lock().unwrap().as_ref())?;
        return Ok(())
    });

    // let _ = std::io::stdin().read_line(&mut data)?;
    // let _ = writeln!(&mut file_handle, "{}", data.as_ref())?;
    // return Ok(())
    thread_handle.join().expect("thread::spawn failed")?;
    Ok(())
}
