mod versions;


fn main() {
    //versions::sync::open_file_and_write("file.txt".to_string()).unwrap();
    versions::aasync::open_file_and_write("file.txt".to_owned()).unwrap();
}