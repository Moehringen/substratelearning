fn main() {
    let triangle = Shape::Triangle(2.0_f32,1.0_f32);
    let square = Shape::Square(5.0_f32);
    let circle = Shape::Circle(2.0_f32,3.1415926_f32);
    println!("the area of triangle is {}", triangle.caculatearea());
    println!("the area of square is {}", square.caculatearea());
    println!("the area of circle is {}", circle.caculatearea());

}

enum Shape<T>{
    Triangle(T,T),
    Square(T),
    Circle(T,T),
}

pub trait Area<T>{
    fn caculatearea(&self) -> T;
}

impl <T> Area<T> for Shape<T>
where
T: std::ops::Mul<Output = T> + Clone + Copy,
{
fn caculatearea(&self) -> T {
        match *self {
            Shape::Circle(a,pi) => a * a * pi ,
            Shape::Square(a) => a * a,
            Shape::Triangle(a, b) => a * b,
        }
    }
}
