

pub mod another_mod {
    pub fn another_function() {
        println!("Another function.");
    }

    fn another_private_function() {
        println!("Another private function.");
    }

    pub fn indirect_access() {
        another_private_function();
    }
}