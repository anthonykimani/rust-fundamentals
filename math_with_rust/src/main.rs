fn main() {
    let a = 2000;
    let b = 4115;
    let result = gcd(a, b);
    println!("The GCD of {a} and {b} is: {}", result);
}

fn gcd(num1: u64, num2: u64) -> u64 {
    if num1 == 0 {
        return num2;
    } else if num2 == 0 {
        return num1;
    } else if num1 > num2 {
        let mut a = num1;
        let mut b = num2;
        while a % b != 0 {
            let temp = a;
            a = b;
            b = temp % a;
        }
        return b;
    } else {
        let mut b = num1;
        let mut a = num2;

        while a % b != 0 {
            let temp = a;
            a = b;
            b = temp % a;
        }
        return b;
    }
}