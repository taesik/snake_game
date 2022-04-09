#[derive(Debug)]
enum PresonId {
  Passport(u32),
  IdentityCard(u32,u32,u32),
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
      id: PersonId::IdentityCard(121,213,121),
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
      String::from("ddd"),
      32,
      PersonId::Passport(13435),
  );

  person.change_age(22);

  println!("{:?}", person.id);
  println!("{:?}",person_2.id);
}


