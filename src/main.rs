pub trait DuckBirdBeak {
    fn quack(&self);
}

pub trait DuckBirdLegs {
    fn move_forward(&self);
    fn move_left(&self);
    fn move_right(&self);
}

pub struct Duck {
    name: String
}

impl DuckBirdBeak for Duck {
    fn quack(&self) {
        println!("Quack! I'am {}!", &self.name)
    }
}

impl DuckBirdLegs for Duck {
    fn move_forward(&self) {
        println!("I move forward!")
    }

    fn move_left(&self) {
        println!("I move left!")
    }

    fn move_right(&self) {
        println!("I move right!")
    }
}

pub fn main() {
    let duck_name = String::from("Chris");
    let duck = Duck { name: duck_name };

    duck.quack();
    duck.move_forward();
    duck.move_left();
    duck.move_right();
}