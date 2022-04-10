use snake_game::learning_rust::{Person, Animal, log_info ,log_info_2};
fn main() {
    let mut person = Person::new();
    let animal = Animal(String::from("cat"));

    
    person.change_age(30);
    log_info(person);
    log_info_2(&person);
}