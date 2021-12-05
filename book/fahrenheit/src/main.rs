use std::io;

fn main() {
    println!("Enter temperature and unit (F or C)!");
    let mut temperature = String::new();
    let mut unit = String::new();

    io::stdin().read_line(&mut temperature).expect("Cannot read line");
    let l = temperature.len()-2;
    let mut unit = temperature.remove(l);
    let reference = 'F';
    let mut test: bool = unit.eq(&reference);
    let mut value: f32 = temperature.trim().parse().expect("Not a number!");

    if test {
        let mut res: f32 = (value - 32.0) * (5.0/9.0);
        let mut res_unit = 'C';
        println!("{} {}", res, res_unit);
    } else {
        let mut res: f32 = value* (9.0/5.0) + 32.0;
        let mut res_unit = 'F';
        println!("{} {}", res, res_unit);
    }
}
