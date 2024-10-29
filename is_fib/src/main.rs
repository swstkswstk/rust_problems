fn main() {
    let _x:i32=1;
    println!("{}",is_fib(5));
}
fn is_fib(num:i32)->i32{
    let mut first=0;
    let mut second=1;
    if num==0{
        return first;
    }
    if num==1{
        return second;
    }
    for _i in 1..num-1{
        let temp=second;
        second=second+first;
        first=temp
    }
    return second;
}