struct User {
    name: String,
    age: i32,
    id_number: i32,
}

impl User {
    pub fn walk(&self) {
        println!("user {} is walking", &self.name);
    }

    pub fn eat(&self) {
        println!("user {} is eating", &self.name);
    }
}
pub fn struct_func() {
    let anto = User{
        name: String::from("Anto"),
        age: 300,
        id_number: 4567894,
    };

    println!("Hello {}", anto.name);
    anto.walk();
    println!("is {}", anto.name);
}

// create a program that finds the area of a rectangle
// the struct should be in a different module
// class assignment: research on the document module and previous assignments should contain documentation