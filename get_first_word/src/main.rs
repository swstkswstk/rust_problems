fn main() {
let str1="Hey this is swstk";
print_type_of(&str1);

println!("{}",get_first_world(str1.to_string()));
}
fn print_type_of<T>(_: &T) {
    println!("The type is: {}", std::any::type_name::<T>());
}
fn get_first_world(sentence:String)->String{
    let mut ans=String::from("");
    for char in sentence.chars(){
        if char==' '{
            break;
        }
        ans.push_str(char.to_string().as_str());
    }
    return ans;
}
