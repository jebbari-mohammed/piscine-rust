use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center : Point,
	pub radius : f64,
}

impl Circle {
   pub fn diameter(self) -> f64 {
    self.radius*2.0
   }
    pub fn new(x: f64,y: f64, z: f64) -> Circle {
        Circle {
        center : Point(x,y),
	    radius : z,
        }
   }
    pub fn area(self) -> f64 {
    self.radius*self.radius*PI
   }
    pub fn intersect(self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance <= (self.radius + other.radius) && distance >= (self.radius - other.radius).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self,point: Point) -> f64 {
        ((self.0 - point.0 ) + (self.1-point.1)).sqrt()
    }
}