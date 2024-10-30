fn main() {
    let my_string = String::from("Hello,World from swstkswstk");
    let length = get_string_len(my_string);
    println!("{}", length);
    let mut x:i16=5;
    println!("{}",x);
    for _ in 1..=100{
        x=x+100;
    }
    println!("{}",x);
    let s="swstk";
    println!("{}",s);
    let greeting=String::from("Hello,World");
    let char1=greeting.chars().nth(11);
    match char1{
        Some (c)=>println!("{}",c),
        None=>println!("No Character found"),
    }
}

fn get_string_len(s: String) -> usize {
    s.chars().count()
}
