extern crate rand;
use rand::Rng;

pub struct Continent {
    pub id: u8,
    pub diseases: DiceHolder,
}

pub struct DiceHolder {
    dice: Vec<Disease>,
}

// partialeq to compare variants
// debug to print the variant for testing
#[derive(Debug, PartialEq)]
pub enum Disease {
    Red,
    Yellow,
    Blue,
    Black,
}

pub struct GameModel {
    pub continents: Vec<Continent>,
    pub cdc: DiceHolder,
    pub treatment_center: DiceHolder,
    pub infection_bag: DiceHolder,
}

impl Continent {
    pub fn new(id: u8) -> Continent {
        Continent {
            id: id,
            diseases: DiceHolder::new(),
        }
    }
    // adds Disease d to the DiceHolder
    pub fn add(&mut self, d: Disease) {
        self.diseases.add(d);
    }
    pub fn disease_count(&self) -> [u8; 4] {
        self.diseases.total_count()
    }
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
    pub fn count(&self, color: &str) -> u8 {
        let mut count = 0;
        match color {
            "red" | "Red" => {
                for i in self.dice.iter() {
                    match i {
                        &Disease::Red => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "yellow" | "Yellow" => {
                for i in self.dice.iter() {
                    match i {
                        &Disease::Yellow => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "blue" | "Blue" => {
                for i in self.dice.iter() {
                    match i {
                        &Disease::Blue => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            "black" | "Black" => {
                for i in self.dice.iter() {
                    match i {
                        &Disease::Black => count += 1,
                        _ => continue,
                    }
                }
                count
            }
            _ => 0,
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
                // TODO cont. i'd like to be able to just return the Option from here
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
    pub fn total_count(&self) -> [u8; 4] {
        [self.count("red"),
         self.count("yellow"),
         self.count("blue"),
         self.count("black")]
    }
}

impl Disease {
    // returns result of die roll - die weights from rule book
    // 0 signifies a CDC token
    pub fn roll(&self) -> u8 {
        let roll = rand::thread_rng().gen_range(0, 6);
        match self {
            &Disease::Red => {
                match roll {
                    0 => 0,
                    1 | 2 => 1,
                    3 => 4,
                    4 | 5 => 6,
                    _ => panic!("invalid random roll"),
                }
            }
            &Disease::Yellow => {
                match roll {
                    0 => 0,
                    1 | 2 => 2,
                    3 => 4,
                    4 | 5 => 5,
                    _ => panic!("invalid random roll"),
                }
            }
            &Disease::Blue => {
                match roll {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 | 5 => 6,
                    _ => panic!("invalid random roll"),
                }
            }
            &Disease::Black => {
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
        self.continents[1].add(Disease::Black);
    }
}