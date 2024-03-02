/*
    Close the provided stream
*/
/// The main function of the program.
///
/// This function serves as the entry point of the program. It prints "Hello, world!" to the console
/// and demonstrates an example usage of the `Person` struct's `age` method.
///
/// # Examples
///
/// ```
/// # struct Person {
/// #     name: String,
/// #     age: u32,
/// # }
/// #
/// # impl Person {
/// #     fn new(name: String, age: u32) -> Self {
/// #         Person { name, age }
/// #     }
/// #
/// #     fn age(&self) -> u32 {
/// #         self.age
/// #     }
/// # }
/// #
/// # fn main() {
/// #     let person = Person::new("Eve".to_string(), 35);
/// #     assert_eq!(person.age(), 35);
/// # }
/// ```
fn main() {
    println!("Hello, world!");

    // Returns the age of the person.
    //
    // # Examples
    //
    // ```
    // let person = Person::new("Eve".to_string(), 35);
    // assert_eq!(person.age(), 35);
    // ```
    println!("Hello, world!");
}
