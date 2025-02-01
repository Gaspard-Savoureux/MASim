use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
    Skin,
};

use super::context::Context;
use super::keymapping::KEY_MAPPINGS;
use super::settings::Settings;

/// set the default style here
pub async fn default_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(BLACK)
        .font_size(16)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font_size(16)
        .text_color(LIGHTGRAY)
        .color(DARKGRAY)
        .build();

    Skin {
        label_style,
        checkbox_style,
        button_style,
        ..root_ui().default_skin()
    }
}

pub fn show_settings(settings: &mut Settings) {
    let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
    root_ui().push_skin(skin);
    settings.refresh_position();

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Settings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.checkbox(hash!(), "Dark theme", &mut settings.dark_theme);
            ui.checkbox(hash!(), "Debug mode", &mut settings.debug);

            // Exit button
            if ui.button(
                vec2(settings.window_size.x - 60., settings.window_size.y - 60.),
                "Close",
            ) {
                settings.toggle_display_settings();
                ui.close_current_window();
            }
        });
}

/// Shows debuging info such as camera position, current screen size, cursor position, etc.
pub fn show_debug_info(ctx: &Context, settings: &Settings) {
    // Current screen size
    draw_text(
        &format!(
            "screen_width: {}, screen_height: {}",
            screen_width(),
            screen_height()
        ),
        10.0,
        40.0,
        20.0,
        settings.text_color,
    );

    // Camera position
    draw_text(
        &format!(
            "camera position (x: {}, y: {})",
            ctx.camera.target.y, ctx.camera.target.y
        ),
        10.0,
        70.0,
        20.0,
        settings.text_color,
    );
}

pub async fn keymappings_skin() -> Skin {
    // let font = load_ttf_font("resources/fonts/Roboto/Roboto-Regular.ttf").await.unwrap();

    let label_style = root_ui()
        .style_builder()
        // .with_font(&font).unwrap()
        .text_color(BLACK)
        .font_size(16)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let group_style = root_ui()
        .style_builder()
        .color_hovered(RED)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    Skin {
        label_style,
        checkbox_style,
        group_style,
        ..root_ui().default_skin()
    }
}

pub fn show_keymapping(settings: &mut Settings) {
    settings.refresh_position();
    let (_, skin) = settings
        .skin
        .get_key_value(&"Keymapping".to_string())
        .unwrap();

    let mut close_clicked = false;

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Keymappings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.push_skin(skin);
            for (key, description) in KEY_MAPPINGS {
                ui.separator();
                ui.group(hash!(key, 1), vec2(280., 60.), |inner_ui| {
                    inner_ui.label(None, key);
                    inner_ui.separator();
                    inner_ui.same_line(20.);
                    inner_ui.label(None, description);
                });
                ui.separator();
            }

            // Exit button
            if ui.button(None, "Close") {
                close_clicked = true;
                ui.close_current_window();
            }

            ui.separator();
            ui.pop_skin();
        });

    if close_clicked {
        settings.toggle_display_keymapping();
    }
}
