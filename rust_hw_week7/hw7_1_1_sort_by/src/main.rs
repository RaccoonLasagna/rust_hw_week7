use std::env;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);
    let mut input = Vec::new();
    for i in &args{
        match i.parse::<i32>(){
            Ok(result) => input.push(result),
            Err(_) => (),
        }
    }
    input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    print!("{:?}", input);
}

use assert_cmd::Command;

#[test]
fn test_sortby2() {
    let mut cmd = Command::cargo_bin("hw7_1_1_sort_by").unwrap();
    cmd.arg("5")
    .arg("6")
    .arg("32")
    .arg("1")
    .arg("3")
    .arg("6")
    .arg("3")
    .assert().success().stdout("[1, 3, 3, 5, 6, 6, 32]");
}