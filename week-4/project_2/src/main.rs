use std::io;

fn main() {
    println!("Enter the experience of the employee (true for experienced, false for inexperienced):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).unwrap();
    let is_experienced: bool = experience_input.trim().parse().expect("Invalid input for experience");

    println!("Enter the age of the employee:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).unwrap();
    let age: u32 = age_input.trim().parse().expect("Invalid input for age");

    let incentive: f64;

    if is_experienced {
        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0;
        } else {
            incentive = 0.0; // Default case if none of the criteria match
        }
    } else {
        incentive = 100_000.0;
    }

    println!("The annual incentive for the employee is: ₦{:.2}", incentive);
}
