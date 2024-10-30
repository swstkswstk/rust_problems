struct Rect{
    length:i16,
    breadth:i16
}
impl Rect{
    fn area(&self)->i16{
        self.length*self.breadth
    }
    fn parameter(&self)->i16{
        2*(self.length+self.breadth)
    }
    fn debug()->i8{
        return 1
    }
}
fn main(){
    let rect1=Rect{
        length:10,
        breadth:20
    };
    println!("Area :{}",rect1.area());
    println!("Parameter :{}",rect1.parameter());
    println!("{}",Rect::debug());

}