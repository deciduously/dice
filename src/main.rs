extern crate rand;
mod controller;
mod model;
mod view;

fn main() {
    let model = model::GameModel::new();
    let view = view::GameView::new();
    let controller = controller::GameController::new(model, view);

    //controller.start_game();
    controller.update_view();
}