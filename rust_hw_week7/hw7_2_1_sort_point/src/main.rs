use std::env;


fn main() {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);
    if args.len() % 2 != 0{
        args.pop();
    }
    //parse string input to int
    let mut input = Vec::new();
    for i in &args{
        match i.parse::<i32>(){
            Ok(result) => input.push(result),
            Err(_) => (),
        }
    }
    //making vector of coordinates
    let mut coordinates = Vec::new();
    for i in 0..(input.len()/2){
        let index1 = i*2;
        let index2 = index1 + 1;
        let tuple = (input[index1], input[index2]);
        coordinates.push(tuple);
    }

    //Ascending X
    coordinates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("Ascending X: {:?}", coordinates);

    //Ascending Y
    coordinates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Ascending Y: {:?}", coordinates);

    //Descending X
    coordinates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("Descending X: {:?}", coordinates);

    //Descending Y
    coordinates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Descending Y: {:?}", coordinates);
}

use assert_cmd::Command;

#[test]
fn test_sortpoint() {
    let mut cmd = Command::cargo_bin("hw7_2_1_sort_point").unwrap();
    cmd.arg("1")
    .arg("6")
    .arg("3")
    .arg("4")
    .arg("5")
    .arg("2")
    .assert().success()
    .stdout("Ascending X: [(1, 6), (3, 4), (5, 2)]
Ascending Y: [(5, 2), (3, 4), (1, 6)]
Descending X: [(5, 2), (3, 4), (1, 6)]
Descending Y: [(1, 6), (3, 4), (5, 2)]
");
}