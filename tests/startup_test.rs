use picatch_lib::filesystem::startup::get_all_files;
use std::path::Path;

#[test]
fn it_lists_all_files() {
    let path = Path::new("./tests/test_photos/");
    let files = get_all_files(&path).unwrap();

    println!("{:?}", &files);

    assert_eq!(5, files.len());
}
