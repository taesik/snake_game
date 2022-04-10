use snake_game::learning_rust::Log;

// use snake_game::learning_rust::{Person, Log, PersonId};
fn main() {

    // let animal = Animal(String::from("cat"));
    // let dog = Dog(String::from("dog"));
    // person.change_age(30);
    // log_info(person);
    // log_info_2(&person);

    // person.display_info();

    let person = snake_game::learning_rust::Person::new();

    // let id = PersonId::Passport(12345);
    // println!(" id : {:?}", id);
    person.display_info();
    println!("{}",person.id);
}