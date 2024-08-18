// Disabiguating between methods with the same name
// Sometimes you'll have methods with the same name

mod supertrait;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Erectus totalus");
    }
}

impl Human {
    fn fly(&self) {
        println!("Fuck this shit");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flying_session() {
        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);
    }
}

// sometimes you have methods that don't require the self 
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn animal_name() {
        let doggo = Dog;

        // This is how you call a function without a self for a struct
        println!("{}", Dog::baby_name());

        // and here is how you call the function related to the trait
        println!("{}", <Dog as Animal>::baby_name());
    }
}
