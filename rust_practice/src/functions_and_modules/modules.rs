pub fn modules() -> String {
    let my_fullname = name_helpers::get_full_name("Shane", "Jones");
    println!("{}", my_fullname);
    return my_fullname;
}
pub mod name_helpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
    }
}
