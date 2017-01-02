use model::Disease;
use model::GameData;
use view::GameView;

pub struct GameController<'a> {
    game: &'a GameData,
    view: &'a GameView<'a>,
}

impl<'a> GameController<'a> {
    pub fn new(game: &'a GameData, view: &'a GameView<'a>) -> GameController<'a> {
        GameController {
            game: game,
            view: view,
        }
    }
    pub fn display(&self) {
      self.view.display();
    }
    
    /*
    pub fn start_game(&mut self) {
      self.game.initial_infect();
    }*/

}