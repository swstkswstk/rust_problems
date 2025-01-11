// fn main() {
//     let mut vec = Vec::new();
//     vec.push(0);
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     println!("{:?}", vec);
//     println!("{:?}", odd_checker(vec));
// }
// fn odd_checker(vec: Vec<i32>) -> Vec<i32> {
//     let mut new_vec=Vec::new();
//     for i in vec{
//         if i%2==1{
//             new_vec.push(i);
//         }
//     }
//     return new_vec;
// }
use std::collections::HashMap;
fn main() {
    let mut vvec = Vec::new();
    vvec.push("d");
    vvec.push("c");
    vvec.push("b");
    vvec.push("a");
    println!("{:?}", vvec);
    // println!("{:?}", filter_even(&vvec));
    // println!("{:?}", vvec);
    let mut users = HashMap::new();
    users.insert(String::from("Hey"), 22);
    users.insert(String::from("SWSTK"), 22);
    users.insert(String::from("October"), 29);
    users.insert(String::from("August"), 25);
    let first_user = users.get("SWSTK");
    match first_user {
        Some(age) => {
            println!("the age is :{}", age);
        }
        None => {
            println!("User not found in db");
        }
    }

    let input_vec = vec![(String::from("Swstk"), 22), (String::from("Hershe"), 22)];
    let hm = groups_value_by_keys(input_vec);
    // let i1 = hm.iter();
    let v1=vec![1,2,3];
    let v1_iter=v1.iter();
    for val in v1_iter{
        println!(" Got: {}",val);
    }

    println!("{:?}", hm);

}
fn groups_value_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn filter_even(vvec: &Vec<String>) -> Vec<String> {
    let new_vec = Vec::new();
    for i in vvec {
        println!("{}", i);
        // if i % 2 == 0 {
        //     new_vec.push(*i);
        // }
    }
    return new_vec;
}
