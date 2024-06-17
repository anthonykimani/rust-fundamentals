pub fn static_array() {
    let rainbow:[&str; 7] = ["Red", "Orange", "Yellow", "Green", "Blue", "Indigo", "Violet", "Violet"];
    for i in rainbow {
        println!("{} is a color visible on rainbows", i);
    }
    println!("The number of colors on a rainbow is {}", rainbow.len());
}