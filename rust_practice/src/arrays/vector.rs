pub fn dynamic_array() {
//     we can use vectors to store values that can be increased and decreased dynamically during runtime.
//     we can create vectors by
//     1. using Vec::new() method
    let _vector: Vec<i64> = Vec::new();

//     2. using macro in Rust
    let macro_vector: Vec<&str> = vec!["Strength", "Hope", "Agility", "Speed"];
    for i in macro_vector {
        println!("{}", i);
    }
}