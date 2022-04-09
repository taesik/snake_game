struct Person {
    name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn new(name: String, last_name: String, age: u32) -> Person {
        Person {
            name,
            last_name,
            age,
        }
    }

    //method
    // first parameter is always self, which represents the instance of the struct the 
    // method is being called on.
    fn display_age(self: &self){
        println!("Current age: {} " ,self.age);
    }
}

fn main() {

  Person::new(name, last_name, age);

  let person = Person {
      name: "Filip".to_string(),
      last_name: "Krul".to_string(),
      age: 32,
  };
  println!("{} {} {}", person.name, person.last_name, person.age);
}


