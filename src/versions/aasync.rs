// NOTE: How much is this code actually async is up to a debate.
// While in this case I may not need Arc, Mutex, Box, etc, I wanted to fell smart and use them.
// I am still learning rust. Do not hate me pls.



use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn open_file_and_write(path: String) -> Result<(), std::io::Error> {
    let data: Arc<Mutex<Box<String>>> = Arc::new(Mutex::new(Box::new(String::new())));

    let mut file_handle = OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(path)?;

    let mutex_for_thread = Arc::clone(&data);

    println!("Enter thing to write in file: ");

    let thread_handle: JoinHandle<Result<(), std::io::Error>> = thread::spawn(move || {
        let _ = std::io::stdin().read_line(&mut mutex_for_thread.lock().unwrap())?;
        let _ = writeln!(&mut file_handle, "{}", mutex_for_thread.lock().unwrap().as_ref());
        return Ok(())
    });

    thread_handle.join().expect("thread::spawn failed")?;
    Ok(())
}