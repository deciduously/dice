use model::game::GameModel;
use view::game::GameView;

pub struct GameController {
    model: GameModel,
    view: GameView,
}

impl GameController {
    pub fn new(model: GameModel, view: GameView) -> GameController {
        GameController {
            model: model,
            view: view,
        }
    }
    pub fn start_game(&mut self) {
        self.model.initial_infect();
    }
    pub fn update_view(&self) {
        self.view.display(&self.model);
    }
}