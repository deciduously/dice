use model::Disease;
use model::GameModel;
use view::GameView;

pub struct GameController {
    model: GameModel,
    view: GameView,
}

impl GameController {
    pub fn new(model: GameModel, view: GameView) -> GameController {
        GameController {
            model: model,
            view: view
        }
    }
    pub fn update_view(self) {
      self.view.display(self.model);
    }
}