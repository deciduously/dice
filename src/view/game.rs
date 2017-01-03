use model::game::GameModel;

pub struct GameView;

impl GameView {
    pub fn new() -> GameView {
        GameView {}
    }
    pub fn display(&self, model: &GameModel) {
        for i in model.continents.iter() {
            println!("Continent {} - {}", i.id, &disease_string(i.total_count()));
        }
        println!("ctr - {}", &disease_string(model.treatment_center.total_count()));
        println!("cdc - {}", &disease_string(model.cdc.total_count()));
        println!("bag - {}", &disease_string(model.infection_bag.total_count()));
    }
}

fn disease_string(c: [u32; 4]) -> String {
    format!("red: {} yellow: {} blue: {} black: {}",
             c[0],
             c[1],
             c[2],
             c[3])
}