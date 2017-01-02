use model::GameModel;

pub struct GameView {
}

impl GameView {
    pub fn new() -> GameView {
        GameView {}
    }
    pub fn display(&self, model: GameModel) {
        for i in model.continents.iter() {
            print!("Continent {} - ", i.id);
            let counts = i.disease_count();
            print!(" red: {} yellow: {} blue {} black {}\n",
                   counts[0],
                   counts[1],
                   counts[2],
                   counts[3]);
        }
        let bag_count = model.infection_bag.total_count();
        println!("bag - red: {} yellow: {} blue: {} black: {}\n",
                 bag_count[0],
                 bag_count[1],
                 bag_count[2],
                 bag_count[3]);
    }
}