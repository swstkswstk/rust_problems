use chrono::{Local,Utc};
fn main() {
    // println!("Hello, world!");
    let now=Utc::now();
    println!("Current date and time in UTC: {}",now);
    let local=Local::now();
    println!("Current date and time in UTC: {}",local);
    let a:i32=2147483647;
    let b:i32=2147483647;
    let sum:i32=a+b;
    println!("{}",sum);
}
