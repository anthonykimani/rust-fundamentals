struct Dog {
    name: String,
    age: u8,
    fur_color: String,
}

struct Person<PetType: Animal> {
     pet: PetType,
}

trait Animal {}

impl Dog {
    fn bark(&self) {
        println!("Bark!");
    }
}

pub fn dog_functionality() {
    let spike = Dog{
        name: String::from("Spiky"),
        age: 16u8,
        fur_color: String::from("Black Coat"),
    };

    spike.bark();
}