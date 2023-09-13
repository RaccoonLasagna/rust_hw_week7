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

    let mut current_f = input[0]; //first field of input
    let max_f = input[1]; //second field of input
    let increment = input[2]; //third field of input

    let mut file = File::create("f2c.html")?;
    file.write_all(
b"<style>
    table, td {
    border: 1px solid #000000;
    border-collapse: collapse;
}
</style>
<table>
  <tr>
    <th>Fahrenheit</th>
    <th>Celcius</th>
  </tr>",)?;

    while current_f <= max_f{
        let current_c = (5./9.)*(current_f - 32.);
        let rounded_c = ((current_c*100.)as f64).round()/100.;
        write!(file,"\n\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>", current_f, rounded_c)?;
        current_f += increment
    }

    let _ = webbrowser::open("f2c.html");
    Ok(())
}
