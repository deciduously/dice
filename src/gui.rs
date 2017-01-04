use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;

use model::game::GameModel;

use std;
use conrod;

pub fn display_console(model: &GameModel) {
    for i in model.continents.iter() {
        println!("Continent {} - {}", i.id, &disease_string(i.total_count()));
    }
    println!("ctr - {}",
             &disease_string(model.treatment_center.total_count()));
    println!("cdc - {}", &disease_string(model.cdc.total_count()));
    println!("bag - {}",
             &disease_string(model.infection_bag.total_count()));
}

widget_ids! {
    pub struct Ids {
        //unique id for each widget
        canvas,
        title,
    }
}

fn disease_string(c: [u32; 4]) -> String {
    format!("red: {} yellow: {} blue: {} black: {}",
            c[0],
            c[1],
            c[2],
            c[3])
}

pub fn theme() -> conrod::Theme {
    conrod::Theme {
        name: "Dice".to_string(),
        padding: conrod::Padding::none(),
        x_position: conrod::Position::Align(conrod::Align::Start, None),
        y_position: conrod::Position::Direction(conrod::Direction::Backwards, 20.0, None),
        background_color: conrod::color::DARK_BLUE,
        shape_color: conrod::color::LIGHT_ORANGE,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: std::collections::HashMap::new(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut GameModel) {
    use conrod::{widget, Positionable, Widget};

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;

    // Canvas holds child widgets
    const TITLE: &'static str = "DICE";
    widget::Canvas::new().pad(MARGIN).set(ids.canvas, ui);

    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);
}