/// As you can see in the lib.rs you could even add examples inside of the documentation. The
/// interesting thing is that these examples are actually run by cargo test.
///
/// ```
/// use cargo_features::Alberto;
/// let alberto = Alberto::Angela(String::from("Alberto"));
/// let string = match alberto {
///     Alberto::Angela(ref s) => s,
///     _ => &String::from("None found"),
/// };
/// assert_eq!(string, &String::from("Alberto"));
/// ```
#[derive(Debug)]
pub enum Alberto {
    Angela(String),
    Carmelo(String),
}
