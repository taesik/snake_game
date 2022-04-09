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
  fn display_age(self){
      println!("Current age: {} " ,self.age);
  }

  fn change_age(&mut self,new_age: u32){
      self.age = new_age;
  }
}

fn main() {

Person::new("name".to_string(), "kim".to_string(), 32);

let mut person = Person {
    name: "Filip".to_string(),
    last_name: "Krul".to_string(),
    age: 32,
};

person.change_age(22);

println!("{} {} {}", person.name, person.last_name, person.age);
person.display_age();
}


