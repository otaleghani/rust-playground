//! This is a package documentation as a comment.

/// This is a documentation as comment. This refers to the next function.
/// \r 
/// Let's see if the new line works or not.
/// cargo doc --open

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;

pub fn main() {
    let color = PrimaryColor::Red;
    color.what_am_i();

    let color_sec = SecondaryColor::Orange;
    color_sec.what_am_i();
}

mod kinds {
    //! This refers to the mod kinds
    
    /// This refers to the next function, enum or struct: PrimaryColor
    #[derive(Debug)]
    pub enum PrimaryColor {
        /// Can I place a comment here?
        Red,
        Blue, 
        Yellow,
    }

    /// # Examples
    /// ```
    /// let color = PrimaryColor::Red;
    /// color.what_am_i();
    /// assert_eq!(color, PrimaryColor::Red);
    /// ```
    /// Here's an example of code highlight
    impl PrimaryColor {
        pub fn what_am_i(&self) {
            println!("I'm {:?}", &self);
        }
    }

    /// This refers to the next function, enum or struct: SecondaryColor
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Purple, 
        Green,
    }

    impl SecondaryColor {
        pub fn what_am_i(&self) {
            println!("I'm {:?}", &self);
        }
    }
}
