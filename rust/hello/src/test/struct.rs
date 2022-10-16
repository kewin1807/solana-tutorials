fn main() {
    let name: String = String::from("Bird");
    let bird = Bird { name, attack: 5 };
    bird.print_name();
    println!("name: {}", bird.name);
}

struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}



//trait => (like implements)
trait Animal {
    fn is_animal(&self) -> bool;
    fn is_fly(&self) -> bool {
        true
    };
}

impl Animal for Bird {
    fn is_fly(&self) -> bool {
        true
    }

    fn is_animal(&self) -> bool {
        false
    }
}
