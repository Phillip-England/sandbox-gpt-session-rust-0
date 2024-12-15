use std::fmt::Debug;

fn main() {
    using_classic_structs();
    using_tuple_structs();
    using_unit_structs();
    using_file_modes(ReadOnly{});
    using_file_modes(WriteOnly{});
    using_singletons();
    using_unit_types_with_traits();
    using_enum_with_match_arm(Message::ChangeColor(222, 222, 201));
    using_traits_to_describe();
}


//=======================================
// TYPES
//=======================================

// classic struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// unit struct
#[derive(Debug)]
struct Marker;

// unit structs as options
#[derive(Debug)]
struct ReadOnly;
#[derive(Debug)]
struct WriteOnly;

// another classic struct
struct Crayon {
    color: String,
}

// a struct which is used as a singleton
struct Logger;
impl Logger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}

// a unit type and a trait, we can implement the trait on the unit type
struct DefaultBehaviour;

trait Action {
    fn perform(&self);
}

impl Action for DefaultBehaviour {
    fn perform(&self) {
        println!("performing the default behaviour!");
    }
}

// an enum which holds multiple values, each with their own unique set of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// a trait that can be implemented on a type to "describe" it
trait Describe {
    fn describe(&self) -> String;
}

struct Animal {
    name: String,
}

struct Vehicle {
    model: String,
}

impl Describe for Animal {
    fn describe(&self) -> String {
        format!("This is an animal named: {}", self.name)
    }
}

impl Describe for Vehicle {
    fn describe(&self) -> String {
        format!("This is a vehicle with the model: {}", self.model)
    }
}

//=======================================
// FUNCTIONS
//=======================================

// creating and printing a classic struct
fn using_classic_structs() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@gmail.com"),
        age: 30,
        active: true,
    };
    println!("{:?}", user);
}

// creating and printing a tuple struct
fn using_tuple_structs() {
    let color = Color(255, 0, 0);
    println!("{:?}", color);
}

// creating and pringing a unit struct
fn using_unit_structs() {
    let marker = Marker{};
    println!("{:?}", marker);
}

// using a unit type to set different 'modes' on a file
// here, I learned how to constrain a generic type to only types with certain traits
fn using_file_modes<T: Debug>(_mode: T) {
    println!("{:?}", _mode);
}

// we can also use unit structs to define singletons
fn using_singletons() {
    let logger = Logger{};
    logger.log("I am using a singleton!");
}

// we can use unit structs to allow us to apply a trait to a type which has no internal data
fn using_unit_types_with_traits() {
    let action = DefaultBehaviour;
    action.perform();
}

// using an enum with a match arm
fn using_enum_with_match_arm(message: Message) {
    match message {
        Message::Quit => println!("quitting!"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, b, g) => println!("Change color to RBG({}, {}, {})", r, b, g),
    }
}

// using traits to describe a few types
fn using_traits_to_describe() {
    let animal = Animal{
        name: String::from("Tiger John"),
    };
    println!("{}", animal.describe());
    let car = Vehicle{
        model: String::from("Honda"),
    };
    println!("{}", car.describe());
}

