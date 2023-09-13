use std::fs::File;
use std::io::prelude::*;
use std::env;
use webbrowser;

fn main() -> std::io::Result<()> {
    //input
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);
    //remove everything but the first 3 fields
    while args.len() > 3{
        args.pop();
    };

    //parse string input to float
    let mut input = Vec::new();
    for i in &args{
        match i.parse::<f32>(){
            Ok(result) => input.push(result),
            Err(_) => input.push(0.),
        }
    }

    let mut current_x = input[0]; //first field of input
    let max_x = input[1]; //second field of input
    let increment = input[2]; //third field of input

    let mut file = File::create("xpo.html")?;
    file.write_all(
b"<style>
    table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
<table>
  <tr>
    <th>x</th>
    <th>x^2</th>
    <th>x^3</th>
  </tr>",)?;

    while current_x <= max_x{
        let x_squared = current_x.powf(2.);
        let x_cubed = current_x.powf(3.);
        write!(file,"\n\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>", current_x, x_squared, x_cubed)?;
        current_x += increment
    }

    let _ = webbrowser::open("xpo.html");
    Ok(())
}
