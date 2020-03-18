use nfd2::Response;

fn main() {
    let result = nfd2::dialog_multiple().open().expect("oh no");

    match result {
        Response::OkayMultiple(files) => println!("File path = {:?}", files),
        Response::Cancel => println!("User canceled"),
        _ => (),
    }
}
