#[cfg(test)]
mod test {
    // rust doesn't have inheritance per se.
    // But the only thing that resambles inheritance is
    // default methods in trait definition

    trait Color {
        fn display_color(&self) {
            println!("I'm a color!");
        }
    }

    struct Rgb {
        r: i32,
        g: i32,
        b: i32,
    }
    impl Color for Rgb {
        fn display_color(&self) {
            println!("R: {}, G: {}, B: {}", self.r, self.g, self.b);
        }
    }

    #[derive(Debug)]
    struct Hex {
        h: String,
    }
    impl Color for Hex {}
    
    #[test]
    fn inheritance() {
        let color: Rgb = Rgb {
            r: 12,
            g: 13,
            b: 14,
        };
        color.display_color();

        let color_hex: Hex = Hex {h: String::from("#ffffff")};
        color_hex.display_color();
        println!("{:?}", color_hex.h);
    }

    // You could use Traits for common behaviors
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Screen {
        // Here we are basically saying that the vec of boxes components
        // contains objects that impl the trait Draw. These objects
        // don't need to be of the same type. To do that we use the
        // dyn (dynamic) keyword.
        // We couldn't achieve this without a trait because with generics
        // the vector would become of one specific type at compile time.
        // Generics are for only one, dynamic traits for multiples.
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("I'm drawing a button of {}x{} with label {}", 
                self.width, 
                self.height, 
                self.label,
            );
        }
    }
    pub struct Select {
        width: u32,
        height: u32,
        option: Vec<String>,
    }
    impl Draw for Select {
        fn draw(&self) {
            let s = self.option.join(", ");
            println!("I'm drawing a select of {}x{} with options {}", 
                self.width, 
                self.height, 
                s,
            );
        }
    }

    #[test]
    fn inheritance_with_traits() {
        let s: Screen = Screen {
            components: vec![
                Box::new(Button {width: 10, height: 20, label: String::from("Alberto")}),
                Box::new(Select {width: 10, height: 20, option: vec![
                    String::from("Alberto"),
                    String::from("Angela"),
                    String::from("Tua madre"),
                ]}),
            ]
        };
        s.run();
    }
}
