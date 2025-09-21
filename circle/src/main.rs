struct Circle{
    radius: f64,
    pi: f64
    
}

impl Circle{
    fn new(radius: f64, pi : f64) -> Circle{
        Circle {radius, pi}
    }
    
    fn area(&self) -> f64{
        self.pi * ((self.radius)*(self.radius))
    }
    
    fn circumference(&self) -> f64{
        2.0 * self.pi * self.radius
    }
}

fn main(){
    let circle = Circle::new(8.0, 3.14);
    println!("Area: { } ", circle.area());
    println!("CIrcumference: { }", circle.circumference());
}
