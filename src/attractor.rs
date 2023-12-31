use crate::point::Point;


pub trait Attractor {
    fn step(&self, p: &mut Point);
}
