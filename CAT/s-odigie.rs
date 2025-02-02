            // Question 1

use std::io;
fn main() {

    println!("Enter candidate details");

    println!("Name: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Email: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    println!("Department: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");

    println!("State of Origin: ");
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Failed to read input");

    println!("Are you a class rep (yes/no): ");
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let is_class_rep = input5.trim().eq_ignore_ascii_case("yes");

    println!("Are you in 100 level (yes/no): ");
    let mut input6 = String::new();
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let is_100_level = input6.trim().eq_ignore_ascii_case("yes");

    println!("What is your CGPA: ");
    let mut input7 = String::new();
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let cgpa:f32 = input7.trim().parse().expect("Failed to input");

    if is_class_rep && !is_100_level && cgpa > 4.0{
        println!("You can vote \nName: {}\nEmail: {}\nDepartment: {}\nState of Origin: {}",
            input1.trim(),
            input2.trim(),
            input3.trim(),
            input4.trim()
        );
    } 
    else {
        println!("Sorry you are not eligible to vote");
    }
}



      
             // Question 2


use std::io;
fn main() {
    
    println!("Enter the staff member's details:");

    println!("Name:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    println!("Number of paper published");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let num_papers: u32 = input2.trim().parse().expect("Invalid input for number of papers");

    let incentive: u32;

    if num_papers >= 3 && num_papers <= 5 {
        incentive = 500_000;
    } else if num_papers > 5 && num_papers < 10 {
        incentive = 800_000;
    } else if num_papers >= 10 {
        incentive = 1_000_000;
    } else if num_papers < 3 {
        incentive = 100_000;
    } else {
        incentive = 0;
    }

    println!("Staff member: {}\n Number of paper: {}\n Incentive: {}", 
        input1.trim(),   
        num_papers,
        incentive
    );
}