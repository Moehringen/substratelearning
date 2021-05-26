fn main() {
    let redlight =  TrafficLight::Red;
    let greenlight =  TrafficLight::Green;
    let yellowlight =  TrafficLight::Yellow;
    println!("time of red light is : {}", redlight.gettime());
    println!("time of green light is : {}", greenlight.gettime());
    println!("time of yellow light is : {}", yellowlight.gettime());
}

enum TrafficLight{
    Red, 
    Green,
    Yellow,
}

pub trait Time{
    fn gettime(&self) -> u8;
}

impl Time for TrafficLight{
    fn gettime(&self) -> u8 {
      match &self{
        TrafficLight::Red => 60,
        TrafficLight::Green => 70,
        TrafficLight::Yellow => 80, 

      }
    }
}