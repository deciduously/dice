extern crate rand;
use rand::Rng;

pub struct Continent {
    pub id: u32,
    pub diseases: DiceHolder,
}

impl Continent {
    pub fn new(id: u32) -> Continent {
        Continent {
            id: id,
            diseases: DiceHolder::new(),
        }
    }
    // adds Disease d to the DiceHolder
    pub fn add(&mut self, d: Disease) {
        self.diseases.add(d);
    }
    pub fn total_count(&self) -> [u32; 4] {
        self.diseases.total_count()
    }
}

pub struct DiceHolder {
    dice: Vec<Disease>,
}

impl DiceHolder {
    pub fn new() -> DiceHolder {
        DiceHolder { dice: Vec::new() }
    }
    // add a Disease to the holder
    pub fn add(&mut self, d: Disease) {
        self.dice.push(d);
    }
    // return number of given color
    pub fn count(&self, color: &str) -> u32 {
        let mut count = 0;
        match color {
            "red" | "Red" => {
                for i in &self.dice {
                    match *i {
                        Disease::Red => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "yellow" | "Yellow" => {
                for i in &self.dice {
                    match *i {
                        Disease::Yellow => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "blue" | "Blue" => {
                for i in &self.dice {
                    match *i {
                        Disease::Blue => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "black" | "Black" => {
                for i in &self.dice {
                    match *i {
                        Disease::Black => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            _ => 0,
        }
    }
    // grab random die, remove and return it
    pub fn grab(&mut self) -> Option<Disease> {
        if self.size() == 0 {
            None
        } else {
            // try randomly until we hit a color that exists
            loop {
                let color = match rand::thread_rng().gen_range(0, 4) {
                    0 => Disease::Red,
                    1 => Disease::Yellow,
                    2 => Disease::Blue,
                    3 => Disease::Black,
                    _ => panic!("Check disease.grab() something's fucked"),
                };
                match self.remove(color) {
                    Some(color) => return Some(color),
                    None => continue,
                }
            }
        }
    }
    // remove and return an Option with the Disease specified - TODO: make this less gross
    // i'm fighting the borrow checker, not working with it
    pub fn remove(&mut self, d: Disease) -> Option<Disease> {
        // grab first die that matches
        let mut found: bool = false;
        let mut index: usize = 0;
        for (i, elem) in self.dice.iter_mut().enumerate() {
            if elem == &d {
                // TODO: cont. i'd like to be able to just return the Option from here
                // but already have a mutable reference to iterate in the first place
                found = true;
                index = i;
                break;
            }
        }
        if found {
            Some(self.dice.remove(index))
        } else {
            None
        }
    }
    // size of holder
    pub fn size(&self) -> usize {
        self.dice.len()
    }
    // returns an array with the counts [red, yellow, blue, black]
    pub fn total_count(&self) -> [u32; 4] {
        [self.count("red"), self.count("yellow"), self.count("blue"), self.count("black")]
    }
}

// partialeq to compare variants
#[derive(Clone, PartialEq)]
pub enum Disease {
    Red,
    Yellow,
    Blue,
    Black,
}

impl Disease {
    // returns result of die roll - die weights from rule book
    // 0 signifies a CDC token
    pub fn roll(&self) -> u32 {
        let roll = rand::thread_rng().gen_range(0, 6);
        match *self {
            Disease::Red => {
                match roll {
                    0 => 0,
                    1 | 2 => 1,
                    3 => 4,
                    4 | 5 => 6,
                    _ => panic!("invalid random roll"),
                }
            }
            Disease::Yellow => {
                match roll {
                    0 => 0,
                    1 | 2 => 2,
                    3 => 4,
                    4 | 5 => 5,
                    _ => panic!("invalid random roll"),
                }
            }
            Disease::Blue => {
                match roll {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 | 5 => 6,
                    _ => panic!("invalid random roll"),
                }
            }
            Disease::Black => {
                match roll {
                    0 => 0,
                    1 | 2 | 3 => 3,
                    4 => 4,
                    5 => 5,
                    _ => panic!("invalid random roll"),
                }
            }
        }
    }
}

pub struct GameModel {
    pub continents: Vec<Continent>,
    pub cdc: DiceHolder,
    pub treatment_center: DiceHolder,
    pub infection_bag: DiceHolder,
}

impl GameModel {
    pub fn new() -> GameModel {
        let mut ret = GameModel {
            continents: Vec::new(),
            cdc: DiceHolder::new(),
            treatment_center: DiceHolder::new(),
            infection_bag: DiceHolder::new(),
        };
        // 6 continents
        for i in 1..7 {
            ret.continents.push(Continent::new(i));
        }
        // 12 of each color in bag
        for _ in 0..12 {
            ret.infection_bag.add(Disease::Red);
            ret.infection_bag.add(Disease::Yellow);
            ret.infection_bag.add(Disease::Blue);
            ret.infection_bag.add(Disease::Black);
        }
        ret
    }
    pub fn initial_infect(&mut self) {
        let mut dice: Vec<Disease> = Vec::new();
        // grab 12 random
        for _ in 0..12 {
            dice.push(self.infection_bag.grab().expect("grab failed during initial_infect"));
        }
        // roll and place
        for i in &dice {
            let mut roll = 0;
            // roll until we don't get a zero
            while roll == 0 {
                roll = i.roll();
            }
            let result = match *i {
                ref i => i.clone(),
            };
            self.continents[roll as usize - 1].add(result);
        }
    }
}
