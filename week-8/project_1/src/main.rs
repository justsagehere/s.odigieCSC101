use std::io;

fn main() {
    println!("Public Service APS Level Checker");
    
    // Prompt for staff role and experience
    println!("Enter the staff role (Office Administrator, Academic, Lawyer, Teacher):");
    let role = get_input();

    println!("Enter the years of experience:");
    let experience: u8 = get_input().trim().parse().unwrap_or(0);

    // Determine APS level based on role and experience
    match role.to_lowercase().as_str() {
        "office administrator" => check_office_administrator_level(experience),
        "academic" => check_academic_level(experience),
        "lawyer" => check_lawyer_level(experience),
        "teacher" => check_teacher_level(experience),
        _ => println!("Invalid role. Please try again."),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn check_office_administrator_level(experience: u8) {
    let level = match experience {
        0..=1 => "APS 1-2 (Intern)",
        2..=5 => "APS 3-5 (Administrator)",
        5..=8 => "APS 5-8 (Senior Administrator)",
        8..=10 => "EL1 8-10 (Office Manager)",
        11..=13 => "EL2 10-13 (Director)",
        _ => "SES (CEO)",
    };
    println!("The staff level is: {}", level);
}

fn check_academic_level(experience: u8) {
    let level = match experience {
        0..=1 => "APS 1-2 (-)",
        2..=5 => "APS 3-5 (Research Assistant)",
        5..=8 => "APS 5-8 (PhD Candidate)",
        8..=10 => "EL1 8-10 (Post-Doc Researcher)",
        11..=13 => "EL2 10-13 (Senior Lecturer)",
        _ => "SES (Dean)",
    };
    println!("The staff level is: {}", level);
}

fn check_lawyer_level(experience: u8) {
    let level = match experience {
        0..=1 => "APS 1-2 (Paralegal)",
        2..=5 => "APS 3-5 (Junior Associate)",
        5..=8 => "APS 5-8 (Associate)",
        8..=10 => "EL1 8-10 (Senior Associate 1-2)",
        11..=13 => "EL2 10-13 (Senior Associate 3-4)",
        _ => "SES (Partner)",
    };
    println!("The staff level is: {}", level);
}

fn check_teacher_level(experience: u8) {
    let level = match experience {
        0..=1 => "APS 1-2 (Placement)",
        2..=5 => "APS 3-5 (Classroom Teacher)",
        5..=8 => "APS 5-8 (Senior Teacher)",
        8..=10 => "EL1 8-10 (Leading Teacher)",
        11..=13 => "EL2 10-13 (Deputy Principal)",
        _ => "SES (Principal)",
    };
    println!("The staff level is: {}", level);
}
