fn main() {
    // Define a list of candidates as a vector of tuples
    let candidates = vec![
        ("Alice", 5),
        ("Bob", 8),
        ("Charlie", 3),
        ("Diana", 10),
        ("Eve", 6),
    ];

    // Find the candidate with the highest years of experience
    let mut best_candidate = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > best_candidate.1 {
            best_candidate = candidate;
        }
    }

    // Display the result
    println!(
        "The person with the highest years of programming experience is {} with {} years.",
        best_candidate.0, best_candidate.1
    );
}
