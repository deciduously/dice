use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;

use model;

use std;
use conrod;

pub fn display_console(model: &model::game::GameModel) {
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
        header,
        footer,
        title,
        body,
        board,
        c1,
        c1c,
        c2,
        c2c,
        c3,
        c3c,
        c4,
        c4c,
        c5,
        c5c,
        c6,
        c6c,
        players,
        infect
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

pub fn gui(ui: &mut conrod::UiCell, ids: &Ids, app: &mut model::game::GameModel) {
    use conrod::{color, widget, Positionable, Labelable, Colorable, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;

    // Canvas holds child widgets
    const TITLE: &'static str = "DICE";
    widget::Canvas::new()
        .flow_down(&[(ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
                     (ids.body,
                      widget::Canvas::new()
                          .length(300.0)
                          .flow_right(&[(ids.board,
                                         widget::Canvas::new()
                                             .pad(20.0)
                                             .flow_down(&[(ids.c1,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN)),
                                                          (ids.c2,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN)),
                                                          (ids.c3,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN)),
                                                          (ids.c4,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN)),
                                                          (ids.c5,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN)),
                                                          (ids.c6,
                                                           widget::Canvas::new()
                                                               .color(color::GREEN))])),
                                        (ids.players, widget::Canvas::new().pad(20.0))])),
                     (ids.footer, widget::Canvas::new().color(color::BLUE))])
        .set(ids.canvas, ui);

    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.header).set(ids.title, ui);
    let button = widget::Button::new().color(color::LIGHT_RED).w_h(30.0,30.0);
    for _click in button.mid_bottom_of(ids.header).set(ids.infect, ui) {
        app.initial_infect();
    }

    widget::Text::new(&disease_string(app.continents[0].total_count()))
        .color(color::RED)
        .middle_of(ids.c1)
        .set(ids.c1c, ui);
    widget::Text::new(&disease_string(app.continents[1].total_count()))
        .color(color::RED)
        .middle_of(ids.c2)
        .set(ids.c2c, ui);
    widget::Text::new(&disease_string(app.continents[2].total_count()))
        .color(color::RED)
        .middle_of(ids.c3)
        .set(ids.c3c, ui);
    widget::Text::new(&disease_string(app.continents[3].total_count()))
        .color(color::RED)
        .middle_of(ids.c4)
        .set(ids.c4c, ui);
    widget::Text::new(&disease_string(app.continents[4].total_count()))
        .color(color::RED)
        .middle_of(ids.c5)
        .set(ids.c5c, ui);
    widget::Text::new(&disease_string(app.continents[5].total_count()))
        .color(color::RED)
        .middle_of(ids.c6)
        .set(ids.c6c, ui);

}