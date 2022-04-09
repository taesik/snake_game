#[derive(Debug)]
enum PersonId {
  Passport(u32),
  IdentityCard(u32,u32,u32),
}

struct Person {
  name: String,
  last_name: String,
  age: u32,
  id: PersonId,
}


struct Animal(String);

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

  fn display_info(&self){
      println!("{} {} {} {:?}" ,self.name,self.last_name,self.age, self.id);
  }
}

fn main() {

  let mut person = Person::new();
  let person_2 = Person::from(
      "Filip".to_string(),
      String::from("ddd"),
      32,
      PersonId::Passport(13435),
  );

  person.change_age(22);
  person.display_info();

  println!("first :  {:?}", person.id);
  println!("second : {:?}",person_2.id);
  
  

//   PersonId::IdentityCard(121,213,121)
  check_person_id(person.id);

  check_person_id(person_2.id);
}

fn check_person_id(id: PersonId){

    // PersonId::IdentityCard(121,213,121)

    if let PersonId::Passport(num) = id {
      println!("It matching Passport {}", num);
    } else {
      println!("It not matching Passport ");
    }


    let result = match id {
        PersonId::IdentityCard(x,y,z)=>{
            println!("Id cardL first value - {}",x+100);    
        },
        PersonId::Passport(x)=>{
            println!("Passport - {}", x);
        },
    };

    let animal = Animal("dog".to_string());

let Animal (animal_type) = animal;

    println!("{}",animal_type);

    println!("{:?}",result);
}
