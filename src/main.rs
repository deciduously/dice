extern crate rand;
mod model;
mod view;
mod controller;

fn main() {
    let game = model::GameData::new();
    let view = view::GameView::new(&game);
    let controller = controller::GameController::new(&game, &view);

    //controller.start_game();
    controller.display();
}