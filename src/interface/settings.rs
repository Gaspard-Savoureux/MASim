use std::collections::HashMap;

use macroquad::{
    color::{Color, BLACK, LIGHTGRAY, WHITE},
    math::{vec2, Vec2},
    ui::Skin,
    window::{clear_background, screen_height, screen_width},
};

use super::ui::default_skin;

pub struct Settings {
    pub display_settings: bool,
    pub display_keymapping: bool,
    pub dark_theme: bool,
    pub debug: bool,
    pub skin: HashMap<String, Skin>,
    pub position: Vec2,
    pub window_size: Vec2,
    pub text_color: Color,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder {
            display_settings: None,
            display_keymapping: None,
            dark_theme: None,
            debug: None,
            skin: None,
            position: None,
            window_size: None,
            text_color: None,
        }
    }

    pub fn refresh_position(&mut self) {
        self.position = vec2(screen_width(), screen_height());
        self.position = self.position / 2. - self.window_size / 2.;
    }

    pub fn toggle_display_settings(&mut self) {
        self.display_settings = !self.display_settings;
    }

    pub fn toggle_display_keymapping(&mut self) {
        self.display_keymapping = !self.display_keymapping;
    }

    pub fn switch_theme(&mut self) {
        self.dark_theme = !self.dark_theme;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    pub fn display_background(&mut self) {
        if self.dark_theme {
            clear_background(BLACK);
            self.text_color = WHITE;
        } else {
            clear_background(LIGHTGRAY);
            self.text_color = BLACK;
        }
    }
}

pub struct SettingsBuilder {
    display_settings: Option<bool>,
    display_keymapping: Option<bool>,
    dark_theme: Option<bool>,
    debug: Option<bool>,
    skin: Option<HashMap<String, Skin>>,
    position: Option<Vec2>,
    window_size: Option<Vec2>,
    text_color: Option<Color>,
}

#[allow(dead_code)]
impl SettingsBuilder {
    pub fn display_settings(mut self, display: bool) -> Self {
        self.display_settings = Some(display);
        self
    }

    pub fn display_keymapping(mut self, display: bool) -> Self {
        self.display_keymapping = Some(display);
        self
    }

    pub fn dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn debug(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn skin(mut self, skin: HashMap<String, Skin>) -> Self {
        self.skin = Some(skin);
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }

    pub fn window_size(mut self, window_size: Vec2) -> Self {
        self.window_size = Some(window_size);
        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub async fn build(self) -> Settings {
        let window_size = self.window_size.unwrap_or(vec2(320., 400.));
        let position = self.position.unwrap_or(vec2(
            screen_width() / 2. - window_size.x,
            screen_height() / 2. - window_size.y,
        ));

        Settings {
            display_settings: self.display_settings.unwrap_or(false),
            display_keymapping: self.display_keymapping.unwrap_or(false),
            dark_theme: self.dark_theme.unwrap_or(false),
            debug: self.debug.unwrap_or(false),
            skin: self.skin.unwrap_or(HashMap::from([(
                "Default".to_string(),
                default_skin().await,
            )])),
            position,
            window_size,
            text_color: if self.dark_theme.unwrap_or(false) {
                WHITE
            } else {
                BLACK
            },
        }
    }
}
