

pub fn test_closures() {
    let add= |x: i8, y: i8| x + y;
    add(-23, 42);

    let print_sum = || {
        println!("{:?}", (add(23,45) + 2i8));
        return add(23, 45) + 2i8;
    };
    print_sum();
}