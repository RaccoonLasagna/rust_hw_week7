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

    //Bubble sort :O
    let n = input.len();
    for i in 0..n-1{
        for j in 0..n-1-(i+1){
            if input[j] > input[j+1]{
                input.swap(j, j+1);
            }
        }
    }
    print!("{:?}", input);
}

use assert_cmd::Command;

#[test]
fn test_sortby2() {
    let mut cmd = Command::cargo_bin("hw7_1_2_sort_loop").unwrap();
    cmd.arg("5")
    .arg("6")
    .arg("32")
    .arg("1")
    .arg("3")
    .arg("6")
    .arg("3")
    .assert().success().stdout("[1, 3, 3, 5, 6, 6, 32]");
}