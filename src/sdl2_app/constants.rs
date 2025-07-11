use std::sync::atomic::AtomicBool;
use sdl2::pixels::Color;

use crate::logic::player::Id;

// Screen dimensions
pub const SCREEN_HEIGHT: u32 = 1080;
pub const SCREEN_WIDTH: u32 = 1920;

// Font sizes
pub const PLAYER_INFO_FONT_SIZE: u16 = 40;
pub const BUTTON_FONT_SIZE: u16 = 40;
pub const SLIDER_FONT_SIZE: u16 = 30;
pub const WRITE_INFO_SIZE: u16 = 40;
pub const START_SCREEN_TEXT_SIZE: u16 = 80;
pub const TITLE_SIZE: u16 = 120;
pub const BUTTON_TEXT_SIZE: u16 = 30;
pub const POT_SIZE: u16 = 40;

// Colors
pub const BACKGROUND_COLOR: Color = Color { r: 200, g: 200, b: 255, a: 0xff };
pub const INFO_B_COLOR: Color = Color { r: 255, g: 102, b: 102, a: 0xff };
pub const BUTTON_COLOR: Color = Color { r: 200, g: 200, b: 200, a: 0xff };
pub const BUTTON_COLOR_PRESSED: Color = Color { r: 100, g: 100, b: 100, a: 0xff };
pub const BALANCE_COLOR: Color = Color { r: 0, g: 0, b: 10, a: 0xff };
pub const FOLDED_COLOR: Color = Color { r: 128, g: 128, b: 128, a: 0xff };
pub const BLACK: Color = Color { r: 0, g: 0, b: 10, a: 0xff };
pub const LIGHT_BLUE: Color = Color { r: 173, g: 216, b: 230, a: 0xff };
pub const RED_COLOR: Color = Color { r: 200, g: 0, b: 0, a: 0xff };
pub const LIGHT_RED: Color = Color { r: 255, g: 105, b: 105, a: 0xff };
pub const DARK_BLUE: Color = Color { r: 0, g: 0, b: 139, a: 255 };
pub const GRAY_COLOR: Color = Color { r: 100, g: 100, b: 100, a: 0xff };
pub const LIGHT_GRAY: Color = Color { r: 200, g: 200, b: 200, a: 0xff };

// Game configuration
pub const MAIN_PLAYER: Id = Id::Player1;

// Asset paths
pub const PATH_TO_FONT: &str = "assets/font/Poppins-Black.ttf";
pub const PATH_TO_DEALER_TOKEN: &str = "assets/dealer_token.png";
pub const PATH_TO_POKERCHIP_RED: &str = "assets/pokerchip_red.png";
pub const PATH_TO_POKERCHIP_YELLOW: &str = "assets/pokerchip_yellow.png";
pub const PATH_TO_POKERCHIP_GREEN: &str = "assets/pokerchip_green.png";
pub const PATH_TO_POKERCHIP_BLUE: &str = "assets/pokerchip_blue.png";

// Button dimensions
pub const START_BUTTON_HEIGHT: u32 = 100;
pub const START_BUTTON_WIDTH: u32 = 600;
pub const BUTTON_WIDTH: u32 = 120;
pub const BUTTON_HEIGHT: u32 = 50;
pub const BUTTON_SPACE: u32 = 10;
pub const BUTTON_END_OF_ROUND_WIDTH: u32 = 200;
pub const BUTTON_END_OF_ROUND_HEIGHT: u32 = 50;

// Card dimensions
pub const CARD_HEIGHT: u32 = 130;
pub const CARD_WIDTH: u32 = 95;
pub const CARD_SIZE: f64 = 3.0;
pub const CARD2_POS: i32 = 30; // relative to first card

// Player UI elements
pub const BALANCE_POS: i32 = 50; // relative to player center, y
pub const BALANCE_WIDTH: u32 = 150;
pub const BALANCE_HEIGHT: u32 = 75;
pub const PLAYER_NAME_POS: (i32, i32) = (25, 85);
pub const PLAYER_NAME_WIDTH: u32 = 150;
pub const PLAYER_NAME_HEIGHT: u32 = 75;

// Slider dimensions
pub const SLIDER_HEIGHT: u32 = 30;
pub const SLIDER_WIDTH: u32 = 300;

// Option screen dimensions
pub const RECT_WIDTH: u32 = 400;
pub const RECT_HEIGHT: u32 = 300;

// Timing constants
pub const ANIMATION_DURATION_MS: u64 = 800;
pub const SHORT_ANIMATION_DURATION_MS: u64 = 200;
pub const FRAME_DURATION_MS: u64 = 33;
pub const BOT_DECISION_DELAY_MS: u64 = 30;

// Chip denominations for display
pub const CHIP_DENOMINATION_LARGE: u32 = 500;
pub const CHIP_DENOMINATION_SMALL: u32 = 100;

// UI positioning offsets
pub const SCREEN_CENTER_Y_OFFSET: i32 = -100;
pub const FOLDED_TEXT_Y_OFFSET: i32 = -100;
pub const BET_DISPLAY_X_OFFSET: i32 = 160;
pub const BET_DISPLAY_Y_OFFSET: i32 = 70;

// Text constants
pub const GAME_TITLE: &str = "MEGA POKER";
pub const START_GAME_TEXT: &str = "START GAME";
pub const SETTINGS_TEXT: &str = "SETTINGS";
pub const EXIT_TEXT: &str = "EXIT";
pub const FOLDED_TEXT: &str = "Folded";
pub const CHIPS_TEXT_PREFIX: &str = "Chips: ";

// Settings constants
pub const SETTINGS_BUTTON_HEIGHT: u32 = 100;
pub const SETTINGS_BUTTON_WIDTH: u32 = 600;
pub const SETTINGS_WINDOW_WIDTH: u32 = 800;
pub const SETTINGS_WINDOW_HEIGHT: u32 = 600;
pub const SETTINGS_FONT_SIZE: u16 = 40;
pub const SETTINGS_TITLE_SIZE: u16 = 60;
pub const PLAYER_COUNT_LABEL: &str = "Number of Players:";
pub const BACK_TEXT: &str = "BACK";
pub const APPLY_TEXT: &str = "APPLY";

// Debug and control
pub static DEBUG: AtomicBool = AtomicBool::new(false);

