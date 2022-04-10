mod another_lib ;
//mod keyword is used for import


use another_lib::another_mod::another_function;
//use keyword is used for variable binding


fn outsider() {
   another_lib::another_mod::another_function();
   println!("outsider");
}



pub mod learning_rust {
    use std::fmt;

    mod top_level {
        pub fn hi (){
            println!("hi"); 
        }

        pub mod low_level {
            pub fn hello() {
                println!("hello");
            }
        }
    }

    pub trait Log {
        fn display_info(&self);
        fn alert_something(&self) {
            println!("Default implementation!!!!!!!")
        }
    }

    pub enum PersonId {
        Passport(u32),
        IndentityCard(u32, u32, u32),
    }

    impl fmt::Display for PersonId {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PersonId::Passport(number) => write!(f, "Passport: {}", number),
                PersonId::IndentityCard(number, month, year) =>{
                    write!(f, "IdentityCard: {}/{}/{}", number, month, year)
                }
            }
        }
    }

    pub struct Person {
        pub name: String, // fields
        last_name: String,
        age: u32,
        pub id: PersonId,
    }

    pub struct Animal(pub String);

    impl Log for Animal {
        fn display_info(&self) {
            println!("{}", self.0)
        }

        fn alert_something(&self) {
        println!("ANIMAL implementation!!!!!!!")
    }
    }

    impl Log for Person {
        fn display_info(&self) {

            /**absolute path**/
            // crate::learning_rust::top_level::hi();
            // crate::learning_rust::top_level::low_level::hello();

            // relative path
            // top_level::hi();
            // top_level::low_level::hello();

            /**
            crate::outsider();
            super -> going outside of current module
            **/
            // super::outsider();


            println!("{} {} {} {}", self.name, self.last_name, self.age, self.id);
        }
    }

    impl Person {
    pub fn new() -> Person {
        Person {
            name: "Default".to_string(),
            last_name: "Default".to_string(),
            age: 0,
            id: PersonId::Passport(540)
        }
    }

    pub fn from(name: String, last_name: String, age: u32, id: PersonId) -> Person {
        Person {
            name,
            last_name,
            age,
            id
        }
    }

        pub fn name(&self) -> &String {
            &self.name
        }


    pub fn change_age(&mut self, new_age: u32) {
    self.age = new_age;
    }
    }

    // impl makes the compiler determine type at the compile time
    // it will create multiple versions of the function, depending on
    // how many types Log trait implements (Person, Animal)
    pub fn log_info(val: impl Log) {
    val.alert_something();
    }


    // dyn is short for dynamic, and says that function should perform dynamic dispatch
    // decission of exactly which function to call at the runtime
    pub fn log_info_2(val: &dyn Log) {
    val.alert_something();
    }

}