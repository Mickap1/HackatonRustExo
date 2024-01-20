pub struct Animal {
    race: String,
    name: String,
    age: i32,
}

impl Animal {
    fn get_race(&self) -> String {
        let race_copy = &self.race;
        return race_copy.to_string()
    }

    fn get_name(&self) -> String {
        let name_copy = &self.name;
        return name_copy.to_string()
    }
}
  
fn main() {
    let cat = Animal {
        name: "Fluppy".to_string(),
        race: "Cat".to_string(),
        age: 2,
    };
  
    println!("This animal is a {} and his name is {}", cat.get_race(), cat.get_name());
}
