#[derive(Debug)]
pub enum AnimalType {
    Fish,
    Birds,
    Mammals,
}
pub trait IFly {
    fn Fly(&self);
}

pub trait AnimalBase {
    fn beforeEat();
    fn Eat(&self) {
        // Default implementation
        self.AfterEat();
    }
    fn AfterEat(&self);
}
#[derive(Debug)]
pub struct Dog {
    pub Type: AnimalType,
    Name: String,
}

impl IFly for Dog {
    fn Fly(&self) {
        println!("I can't fly, sorry")
    }
}

impl AnimalBase for Dog {
    fn beforeEat() {
        // Dog SHOULD implement this function
    }
    fn AfterEat(&self) {
        // Dog SHOULD implement this function
    }
    fn Eat(&self) {
        // It is optional
    }
}

impl Dog {
    // Constructor  ||  Static Methods
    fn new(name: String) -> Self {
        Dog {
            Type: AnimalType::Mammals,
            Name: name,
        }
    }

    // function on Dog object
    fn WhatIsMyName(&self) -> &str {
        &self.Name
    }
}

fn main() {
    let dog = Dog::new("Happi".to_string());
    let myNameIs = dog.WhatIsMyName();
    dog.Fly();
    println!("{:?} {}", dog, myNameIs);
}
