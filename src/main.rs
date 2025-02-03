use std::collections::HashMap;

use interface::{
    context::Context,
    grid::{Grid, GridSize},
    keymapping::apply_input,
    settings::Settings,
    ui::{default_skin, keymappings_skin, show_debug_info, show_keymapping, show_settings},
};
use macroquad::{prelude::*, ui::root_ui};

pub mod interface;

#[macroquad::main("MASim")]
async fn main() {
    let mut settings = Settings::builder()
        .skin(HashMap::from([
            ("Default".to_string(), default_skin().await),
            ("Keymapping".to_string(), keymappings_skin().await),
        ]))
        .build()
        .await;

    let mut camera =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));

    let mut ctx = Context {
        grid_size: vec2(8., 8.),
        camera,
    };

    let mut grid = Grid::new(
        vec2(screen_width() * 0.1, screen_height() * 0.1),
        GridSize {
            width: 16,
            heigth: 16,
        },
        None,
    );

    loop {
        settings.display_background();

        // User input
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        apply_input(&mut ctx, &mut settings);

        // 2D context
        set_default_camera();

        grid.display(
            vec2(screen_width() * 0.1, screen_height() * 0.1),
            32.,
            settings.text_color,
        );

        // Buttons
        let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
        root_ui().push_skin(skin);
        if root_ui().button(vec2(screen_width() - 80., 20.), "Settings  ") {
            settings.toggle_display_settings();
        }

        if root_ui().button(vec2(screen_width() - 80., 40.), "Keymapping") {
            settings.toggle_display_keymapping();
        }

        root_ui().pop_skin();

        #[cfg_attr(any(), rustfmt::skip)]
        { // Display settings related informations
        if settings.display_settings   { show_settings(&mut settings); }
        if settings.display_keymapping { show_keymapping(&mut settings); }
        if settings.debug { show_debug_info(&ctx, &settings); }
        }

        next_frame().await
    }
}
