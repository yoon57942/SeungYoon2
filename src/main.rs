use std::f32::consts::PI;

struct Object {
    radius: u32,
}
impl Object {
    fn area(&self) -> f32 {
        4.0* PI * (self.radius.pow(2) as f32)
    }
    fn new(radius: u32) -> Object {
        Object {
            radius
        }
    }
    fn show(&self) {
        println!("{} radius sphere area :{}",self.radius, self.area());
    }
}
fn main() {
    let o: Object = Object::new(10);

    o.show();
    
}