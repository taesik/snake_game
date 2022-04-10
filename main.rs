use snake_game::learning_rust::{Person, Log, PersonId};
fn main() {

    // let animal = Animal(String::from("cat"));

    
    // person.change_age(30);
    // log_info(person);
    // log_info_2(&person);

    // person.display_info();

    let mut person = Person::new();
    let id = PersonId::Passport(12345);
    println!(" id : {:?}", id);

    println!("person name : {}",person.name());

}