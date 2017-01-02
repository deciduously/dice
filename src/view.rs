use model::GameData;

pub struct GameView<'a> {
    pub game: &'a GameData,
}

impl<'a> GameView<'a> {
    pub fn new(game: &'a GameData) -> GameView<'a> {
        GameView { game: game }
    }
    pub fn display(&self) {
        for i in self.game.continents.iter() {
            print!("Continent {} - ", i.id);
            let counts = i.disease_count();
            print!(" red: {} yellow: {} blue {} black {}\n",
                   counts[0],
                   counts[1],
                   counts[2],
                   counts[3]);
        }
        let bag_count = self.game.infection_bag.total_count();
        println!("bag - red: {} yellow: {} blue: {} black: {}\n",
                 bag_count[0],
                 bag_count[1],
                 bag_count[2],
                 bag_count[3]);
    }
}