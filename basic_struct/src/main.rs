struct User{
    first_name:String,
    last_name:String,
    age:i16

}
fn main(){
    let user=User{
        first_name:String::from("swstk"),
        last_name:String::from("swstk"),
        age:22,
    };
    println!("First Name:{} \nLast Name:{} \nAge:{}",user.first_name,user.last_name,user.age);
}