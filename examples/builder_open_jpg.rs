use nfd2::Response;

fn main() {
    let result = nfd2::dialog().filter("jpg").open().expect("oh no");

    match result {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::Cancel => println!("User canceled"),
        _ => (),
    }
}
