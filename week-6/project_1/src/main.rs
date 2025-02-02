use std::io;

fn main() {
    println!("Select a calculation to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => calculate_trapezium_area(),
        2 => calculate_rhombus_area(),
        3 => calculate_parallelogram_area(),
        4 => calculate_cube_area(),
        5 => calculate_cylinder_volume(),
        _ => println!("Invalid choice! Please run the program again."),
    }
}

fn calculate_trapezium_area() {
    let (height, base1, base2) = get_three_inputs("height", "base1", "base2");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is {:.2}", area);
}

fn calculate_rhombus_area() {
    let (diagonal1, diagonal2) = get_two_inputs("diagonal1", "diagonal2");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is {:.2}", area);
}

fn calculate_parallelogram_area() {
    let (base, altitude) = get_two_inputs("base", "altitude");
    let area = base * altitude;
    println!("The area of the parallelogram is {:.2}", area);
}

fn calculate_cube_area() {
    let side = get_single_input("length of the side");
    let area = 6.0 * side * side;
    println!("The area of the cube is {:.2}", area);
}

fn calculate_cylinder_volume() {
    let (radius, height) = get_two_inputs("radius", "height");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is {:.2}", volume);
}

fn get_single_input(prompt: &str) -> f64 {
    println!("Enter {}:", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_two_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
    let val1 = get_single_input(prompt1);
    let val2 = get_single_input(prompt2);
    (val1, val2)
}

fn get_three_inputs(prompt1: &str, prompt2: &str, prompt3: &str) -> (f64, f64, f64) {
    let val1 = get_single_input(prompt1);
    let val2 = get_single_input(prompt2);
    let val3 = get_single_input(prompt3);
    (val1, val2, val3)
}
