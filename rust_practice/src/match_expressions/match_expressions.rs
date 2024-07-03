
pub fn match_expressions() {
    let myage = 35;

    match myage {
        1..=9 | 10..=20 => {
            println!("Your age is 17");
        }
        18..=80 => {
            println!("Your age is 18 to 80");
        }
        _ => {

        }
    }

}