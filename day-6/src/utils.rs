// Helpers

pub fn format_input(input: Vec<String>) -> Vec<Fish> {
    let mut fish: Vec<Fish> = Vec::new();
    for i in input {
        fish.push(Fish::new(i.parse::<u8>().unwrap()));
    }
    return fish
}

// Structs

#[derive(Debug)]
pub struct Fish {
    pub days: u8
}

impl Fish {
    pub fn new(days: u8) -> Fish {
        return Fish{days};
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) -> Option<Fish> {
        if self.days == 0 {
            self.days = 6;
            return Some(Fish::new(8));
        } else {
            self.days -= 1;
            return None;
        }
    }
}