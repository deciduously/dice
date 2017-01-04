#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate rand;

mod model;
mod gui;

use conrod::backend::piston::{window, Window, WindowEvents, OpenGL};
use conrod::backend::piston::event::UpdateEvent;


fn main() {
    const WIDTH: u32 = 800 ;
    const HEIGHT: u32 = 600;

    let mut model = model::game::GameModel::new();

    let opengl = OpenGL::V3_2;
    let mut window: Window = window::WindowSettings::new("Canvas Demo", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();
    let mut events = WindowEvents::new();
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64])
        .theme(gui::theme())
        .build();

    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    let mut text_texture_cache = window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    let image_map = conrod::image::Map::new();
    let ids = gui::Ids::new(ui.widget_id_generator());

    // poll events from the window
    while let Some(event) = window.next_event(&mut events) {
        // convert piston event to conrod event
        if let Some(e) = window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }

        event.update(|_| {
            let mut ui = ui.set_widgets();
            gui::gui(&mut ui, &ids, &mut model);
        });

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T {
                    img
                };
                window::draw(c,
                             g,
                             primitives,
                             &mut text_texture_cache,
                             &image_map,
                             texture_from_image);
            }
        });
    }
}