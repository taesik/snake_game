#[derive(Debug)]
enum PresonId {
  Passport,
  IdentityCard,
}

struct Person {
  name: String,
  last_name: String,
  age: u32,
  id: PersonId,
}

impl Person {
  fn new()->Person {
    Person { 
      name: "default".to_string(),
      age: 13,
      last_name: "default".to_string(),
      id: PersonId::IdentityCard,
    }
  }

  fn from(name: String, last_name: String,age: u32,id: PersonId) -> Person {
    Person { 
      name,
      age,
      last_name,
      id,  
    }
  }

  //method
  // first parameter is always self, which represents the instance of the struct the 
  // method is being called on.
  fn display_age(self){
      println!("Current age: {} " ,self.age);
  }

  fn change_age(&mut self,new_age: u32){
      self.age = new_age;
  }
}

fn main() {

Person::new("name".to_string(), "kim".to_string(), 32);


let mut person = Person::new();
let person_2 = Person::from(
    "Filip".to_string(),
    "Krul".to_string(),
    32,
    PersonId::Passport,
);

person.change_age(22);

println!("{} {} {}", person.name, person.last_name, person.age);
println!("{}",person.id);
}


