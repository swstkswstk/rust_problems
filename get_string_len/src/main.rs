fn main() {
    let my_string = String::from("Hello,World from swstkswstk");
    let length = get_string_len(my_string);
    println!("{}", length);
}

fn get_string_len(s: String) -> usize {
    s.chars().count()
}
