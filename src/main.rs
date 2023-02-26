use std::{error::Error, io};
fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter a number: ");
    let num1 = input!(i32);
    println!("Enter a number: ");
    let num2 = input!(i32);
    println!("Enter operation(+, -, *, /, %): ");
    let op = input!(char);
    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '%' => num1 % num2,
        _ => panic!("Invalid operation"),
    };
    println!("{} {} {} = {}", num1, op, num2, result);
    Ok(())
}

#[macro_export]
macro_rules! input {
    ($($type: ty),+) => {($({
        let mut buf = String::default();
        io::stdin().read_line(&mut buf)?;
        buf.trim().parse::<$type>()?
    }),+)};
}
