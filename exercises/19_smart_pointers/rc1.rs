use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    let sun = Rc::new(Sun);

    let mercury = Planet::Mercury(Rc::clone(&sun));
    let venus = Planet::Venus(Rc::clone(&sun));

    mercury.details();
    venus.details();

    println!("strong count = {}", Rc::strong_count(&sun));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6
        jupiter.details();

        let saturn = Planet::Saturn(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7
        saturn.details();

        let uranus = Planet::Uranus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8
        uranus.details();

        let neptune = Planet::Neptune(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4

        drop(earth);
        println!("reference count = {}", Rc::strong_count(&sun)); // 3

        drop(venus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 2

        drop(mercury);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
