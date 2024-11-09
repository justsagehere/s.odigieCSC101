use std::io;

fn main() {
    println!("Enter the coefficients a, b, and c of the quadratic equation ax^2 + bx + c = 0:");

    // Input coefficients a, b, c
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    io::stdin().read_line(&mut a_input).unwrap();
    io::stdin().read_line(&mut b_input).unwrap();
    io::stdin().read_line(&mut c_input).unwrap();

    let a: f64 = a_input.trim().parse().expect("Invalid input for a");
    let b: f64 = b_input.trim().parse().expect("Invalid input for b");
    let c: f64 = c_input.trim().parse().expect("Invalid input for c");

    if a == 0.0 {
        println!("This is not a quadratic equation since a = 0.");
        return;
    }

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of the roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots: root1 = {} and root2 = {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The equation has one real root: root = {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}

