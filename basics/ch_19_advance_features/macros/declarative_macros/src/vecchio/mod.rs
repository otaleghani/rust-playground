pub mod vecchio {
    // here we are exporting this macro
    #[macro_export]
    // we are givin it a name
    macro_rules! vecchio {
        // we are expecting some parameters
        ( $( $x:expr ),* ) => {
            {
                // we are defining the code that it has to spit out
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
}
