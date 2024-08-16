#[cfg(test)]
mod tests {
    // Rust has some OOP features, like encapsulation.
    pub struct AveragedCollection {     // public object
        list: Vec<i32>,                 // private fields
        average: f64,
    }

    impl AveragedCollection {           // public api
        pub fn new() -> Self {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {  // internal private logic
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    // by doins something like this we are basically protecting 
    // the integrity of the data.

    #[test]
    fn encapsulation() {
        //let mut collection: AveragedCollection = AveragedCollection{list: vec![], average: 0.0};
        let mut collection = AveragedCollection::new();

        let add = 12;
        collection.add(add);
        println!("added {}, average {}", add, collection.average());

        let add = 24;
        collection.add(add);
        println!("added {}, average {}", add, collection.average());

        let add = 48;
        collection.add(add);
        println!("added {}, average {}", add, collection.average());

        let popped = collection.remove();
        println!("popped {}, average {}", popped.unwrap(), collection.average());
    }
}
