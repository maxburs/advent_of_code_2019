use std::fs;

fn fuel_required(mass: i32) -> i32 {
    let fuel = ((mass as f64) / 3.0).floor() as i32 - 2;
    if fuel <= 0 {
        return 0;
    }
    return fuel + fuel_required(fuel);
}

fn main() -> Result<(), std::io::Error> {
    let file = fs::read_to_string("./day1.txt")?;

    let fuel = file
      .split('\n')
      .map(|s| s.trim())
      .filter(|s| *s != "")
      .map(|s| fuel_required(s.parse().unwrap()));

    println!("{}", fuel.sum::<i32>());
    Ok(())
}
