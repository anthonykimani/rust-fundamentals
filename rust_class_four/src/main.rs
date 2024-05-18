mod divider;
use divider::divide;



fn main() {
    // let num_array: [char; 7] = ['🧪', '😂', '👌', '🎇', '🧪', '😒', '🧪'];
    // for i in num_array {
    //     println!("{}", i);
    // }
    //
    // let (one, two) = slice();
    // println!("{}", one);
    // fibonacci(3);
    // let mut superman = create_game_character("Clark Kent", 5, "novice");
    // superman.play(5);
    divide::divide();
}

fn slice() -> (i32, String) {
    let text = "web3clubs";
    return (2, text.to_string());
}

fn fibonacci(num: i32) -> i32 {
    // F(n)=F(n−1)+F(n−2)
    if num <= 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        let fib = (num+1) + (num+2);
        println!("{}", fib);
        return fib;
    }
}


// using knowledge of rust structs implement a character with name, score, level

struct Game_Character {
    name: String,
    score: i32,
    level: String,
}

impl Game_Character {
    fn play(&mut self, even_num: i32) {
        if even_num % 2 == 0 {
            self.score+= 1;
        } else {
            self.score-= 1;
        }
        println!("{}", self.score)
    }

}

fn create_game_character(_name: &str, _score: i32, _level: &str) -> Game_Character {
    let character = Game_Character {
        name: String::from(_name),
        score: _score,
        level: String::from(_level),
    };
    println!("Your game character is {:?}", character.name);
    return character;
}

// create a program that takes in an input from the terminal and then takes in a second input from the terminal divides the first input by the second input,
// then test the program for extreme conditions like divide by 0, integer overflow, inputting a letter
//  write tests or test manually