//! The Menu and User Interface system.
//!
//! This module manages all the non-gameplay screens, such as the Main Menu,
//! the Pause Screen, and the Game Over screen. It handles drawing buttons and text,
//! processing mouse clicks, and taking care of saving and loading the player's progress.

use std::collections::HashMap;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window};
use std::fs;

use crate::GameAction;
use crate::audio::audio_handler::Audio;
use crate::game::gamestate::Game;
use crate::keys::key_assigned::save_key_bindings_in_file;
use crate::render::RendererData;

const SCREEN_WIDTH: usize = 800;
const SCREEN_HEIGHT: usize = 450;

const COL_BG_MAIN: u32  = 0x111111;
const COL_BG_GAMEOVER: u32 = 0x330000;
const COL_BG_KEYCHANGE: u32 = 0x002244;
const COL_BG_PAUSED: u32 = 0x002244;
const COL_TITLE_RED: u32 = 0xCC0000;
const COL_TITLE_BLUE: u32 = 0x44AAFF;
const COL_BTN_NORMAL: u32 = 0x444444;
const COL_BTN_HOVER: u32 = 0xAA0000;
const COL_TEXT_WHITE: u32 = 0xFFFFFF;

/// Dictates the high-level routing in the `main.rs` loop.
pub enum AppState {
    StartScreen,
    Playing,
    GameOver,
    GamePaused,
    Quit,
    KeyChangeScreen,
}

/// Simple layout container for button definitions.
pub struct ButtonDef {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    text: String,
}

/// Holds the minimal state required for the IMGUI.
/// Since `minifb` natively handles keyboard debouncing via `KeyRepeat::No`,
/// we only need to manually track the mouse state for edge detection (clicks).
pub struct Menu {
    was_mouse_down: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            was_mouse_down: false,
        }
    }

    /// Renders the main menu and handles user interactions.
    pub fn update_and_draw_start_menu(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
        renderer_data: &RendererData,
        audio: &mut Audio,
    ) -> AppState {
        // Clear the buffer from the previous frame
        buffer.fill(COL_BG_MAIN);

        // Fetch user inputs
        let (mx, my, clicked) = self.get_mouse_state(window);
        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);
        let click2 = window.is_key_pressed(Key::Key2, KeyRepeat::No);
        let click3 = window.is_key_pressed(Key::Key3, KeyRepeat::No);
        let click4 = window.is_key_pressed(Key::Key4, KeyRepeat::No);

        // Render titles
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "DOOMSQUAD",
            50,
            6,
            COL_TITLE_RED,
        );
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "MAIN MENU",
            105,
            2,
            COL_TEXT_WHITE,
        );

        // Define button layouts
        let btn_start = ButtonDef {
            x: 230,
            y: 160,
            w: 340,
            h: 40,
            text: "[1] NEW GAME".to_string(),
        };
        let btn_load = ButtonDef {
            x: 230,
            y: 210,
            w: 340,
            h: 40,
            text: "[2] LOAD MAP".to_string(),
        };

        let audio_text = if !audio.is_muted {
            "[3] SOUND: ON"
        } else {
            "[3] SOUND: OFF"
        };

        let btn_audio = ButtonDef {
            x: 230,
            y: 260,
            w: 340,
            h: 40,
            text: audio_text.to_string(),
        };

        let btn_keys = ButtonDef {
            x: 230,
            y: 310,
            w: 340,
            h: 40,
            text: "[4] CHANGE KEYS".to_string(),
        };

        let btn_quit = ButtonDef {
            x: 230,
            y: 360,
            w: 340,
            h: 40,
            text: "[ESC] QUIT GAME".to_string(),
        };

        // Interaction processing
        if self.draw_flat_button(buffer, &btn_start, mx, my) && clicked || click1 {
            *game = Game::new_game(renderer_data).unwrap(); // ! this unwrap is intentionally accepted
            game.map_swap(renderer_data, 1); // Jump straight to map 1 (map 0 is reserved for testing)
            return AppState::Playing;
        }

        #[allow(clippy::collapsible_if)]
        if self.draw_flat_button(buffer, &btn_load, mx, my) && clicked || click2 {
            // Read saved map index from disk and safely parse it
            if let Ok(content) = fs::read_to_string("savegame.txt")
                && let Ok(index) = content.trim().parse::<usize>()
            {
                *game = Game::new_game(renderer_data).unwrap(); // ! this unwrap is intentionally accepted
                game.map_swap(renderer_data, index);
                return AppState::Playing;
            }
        }


        if self.draw_flat_button(buffer, &btn_keys, mx, my) && clicked || click4 {
            return AppState::KeyChangeScreen;
        }

        if self.draw_flat_button(buffer, &btn_quit, mx, my) && clicked {
            return AppState::Quit;
        }

        if self.draw_flat_button(buffer, &btn_audio, mx, my) && clicked || click3 {
            audio.set_muted(!audio.is_muted);
            return AppState::StartScreen;
        }

        // Keep looping in the menu until an action changes the state
        AppState::StartScreen
    }

    /// Renders the death screen and allows the user to save their current level progress.
    pub fn update_and_draw_game_over(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
    ) -> AppState {
        buffer.fill(COL_BG_GAMEOVER);

        let (mx, my, clicked) = self.get_mouse_state(window);
        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);

        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "YOU DIED",
            80,
            6,
            COL_TITLE_RED,
        );

        let btn_save = ButtonDef {
            x: 230,
            y: 220,
            w: 340,
            h: 60,
            text: "[1] SAVE MAP INDEX".to_string(),
        };
        let btn_menu = ButtonDef {
            x: 230,
            y: 300,
            w: 340,
            h: 60,
            text: "[ESC] MAIN MENU".to_string(),
        };

        if self.draw_flat_button(buffer, &btn_save, mx, my) && clicked || click1 {
            // Persist the current level index to disk so it can be loaded later
            let _ = fs::write("savegame.txt", game.map_index.to_string());
            return AppState::StartScreen;
        }

        if self.draw_flat_button(buffer, &btn_menu, mx, my) && clicked {
            return AppState::StartScreen;
        }

        AppState::GameOver
    }

    /// Renders the pause menu (triggered by pressing ESC during gameplay).
    pub fn update_and_draw_game_paused(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        game: &mut Game,
        audio: &mut Audio,
    ) -> AppState {
        buffer.fill(COL_BG_PAUSED);

        let (mx, my, clicked) = self.get_mouse_state(window);

        let click1 = window.is_key_pressed(Key::Key1, KeyRepeat::No);
        let click2 = window.is_key_pressed(Key::Key2, KeyRepeat::No);
        let click3 = window.is_key_pressed(Key::Key3, KeyRepeat::No);

        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "GAME PAUSED",
            60,
            6,
            COL_TITLE_BLUE,
        );

        let audio_text = if !audio.is_muted {
            "[3] SOUND: ON"
        } else {
            "[3] SOUND: OFF"
        };

        let btn_resume = ButtonDef {
            x: 230,
            y: 150,
            w: 340,
            h: 50,
            text: "[1] RESUME GAME".to_string(),
        };
        let btn_save = ButtonDef {
            x: 230,
            y: 210,
            w: 340,
            h: 50,
            text: "[2] SAVE & EXIT".to_string(),
        };
        let btn_audio = ButtonDef {
            x: 230,
            y: 270,
            w: 340,
            h: 50,
            text: audio_text.to_string(),
        };
        let btn_menu = ButtonDef {
            x: 230,
            y: 330,
            w: 340,
            h: 50,
            text: "[ESC] MAIN MENU".to_string(),
        };

        if self.draw_flat_button(buffer, &btn_resume, mx, my) && clicked || click1 {
            // Simply return to the Playing state without touching the game object to resume seamlessly
            return AppState::Playing;
        }

        if self.draw_flat_button(buffer, &btn_save, mx, my) && clicked || click2 {
            let _ = std::fs::write("savegame.txt", game.map_index.to_string());
            return AppState::StartScreen;
        }

        if self.draw_flat_button(buffer, &btn_menu, mx, my) && clicked {
            return AppState::StartScreen;
        }

        if self.draw_flat_button(buffer, &btn_audio, mx, my) && clicked || click3 {
            audio.set_muted(!audio.is_muted);
            return AppState::GamePaused;
        }

        AppState::GamePaused
    }

    /// Renders the keychange menu (triggered by pressing 4 in the menu).
    pub fn update_and_draw_key_change(
        &mut self,
        window: &mut Window,
        buffer: &mut [u32],
        audio: &mut Audio,
        key_map: &mut HashMap<GameAction, Key>,
        //game: &mut Game,
    ) -> AppState {
        buffer.fill(COL_BG_KEYCHANGE);

        let (mx, my, clicked) = self.get_mouse_state(window);

        let click_esc = window.is_key_pressed(Key::Escape, KeyRepeat::No);

        
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "CHANGE KEY BINDINGS",
            25,
            4,
            COL_TITLE_BLUE,
        );

        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Click on any gameplay action, then",
            70,
            2,
            COL_TITLE_BLUE,
        );
        Self::draw_text_centered(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "press the key you want to bind the action to.",
            90,
            2,
            COL_TITLE_BLUE,
        );

        let btn_forwards = ButtonDef {
            x: 80, y: 130,
            w: 290, h: 25,
            text: "Forwards".to_string(),
        };
        let btn_left = ButtonDef {
            x: 420, y: 130,
            w: 290, h: 25,
            text: "Left".to_string(),
        };
        let btn_backwards = ButtonDef {
            x: 80, y: 160,
            w: 290, h: 25,
            text: "Backwards".to_string(),
        };
        let btn_right = ButtonDef {
            x: 420, y: 160,
            w: 290, h: 25,
            text: "Right".to_string(),
        }; //
        let btn_sprint = ButtonDef {
            x: 80, y: 190,
            w: 290, h: 25,
            text: "Sprint".to_string(),
        };
        let btn_jump = ButtonDef {
            x: 420, y: 190,
            w: 290, h: 25,
            text: "Jump".to_string(),
        };
        let btn_crouch = ButtonDef {
            x: 80, y: 220,
            w: 290, h: 25,
            text: "Crouch".to_string(),
        };
        let btn_rocket_jump = ButtonDef {
            x: 420, y: 220,
            w: 290, h: 25,
            text: "Rocket Jump".to_string(),
        }; //
        let btn_interact = ButtonDef {
            x: 80, y: 250,
            w: 290, h: 25,
            text: "Interact".to_string(),
        };
        let btn_toggle_fine_aiming = ButtonDef {
            x: 420, y: 250,
            w: 290, h: 25,
            text: "Toggle Fine Aiming".to_string(),
        };
        let btn_move_crosshair_up= ButtonDef {
            x: 80, y: 280,
            w: 290, h: 25,
            text: "Crosshair Up".to_string(),
        };
        let btn_turn_left = ButtonDef {
            x: 420, y: 280,
            w: 290, h: 25,
            text: "Turn Left".to_string(),
        }; //
        let btn_move_crosshair_down = ButtonDef {
            x: 80, y: 310,
            w: 290, h: 25,
            text: "Crosshair Down".to_string(),
        };
        let btn_turn_right= ButtonDef {
            x: 420, y: 310,
            w: 290, h: 25,
            text: "Turn Right".to_string(),
        };
        let btn_fire_bullet = ButtonDef {
            x: 80, y: 340,
            w: 290, h: 25,
            text: "Fire Bullet".to_string(),
        };
        let btn_fire_arrow= ButtonDef {
            x: 420, y: 340,
            w: 290, h: 25,
            text: "Fire Arrow".to_string(),
        }; //
        let btn_enter_godmode = ButtonDef {
            x: 80, y: 370,
            w: 290, h: 25,
            text: "Enter Godmode".to_string(),
        };
        let btn_lock_enemies = ButtonDef {
            x: 420, y: 370,
            w: 290, h: 25,
            text: "Lock All Enemies".to_string(),
        };
        let btn_pause_game = ButtonDef {
            x: 80, y: 400,
            w: 290, h: 25,
            text: "Pause Game".to_string(),
        };

        // #[allow(clippy::collapsible_if)]
        // if self.draw_flat_button(buffer, &btn_load, mx, my) && clicked {
        //     // Read saved map index from disk and safely parse it
        //     if let Ok(content) = fs::read_to_string("keysAction.txt")
        //         && let Ok(index) = content.trim().parse::<usize>()
        //     {
        //         *game = Game::new_game(renderer_data).unwrap(); // ! this unwrap is intentionally accepted
        //         game.map_swap(renderer_data, index);
        //         return AppState::Playing;
        //     }
        // }
        // 
        // if self.draw_flat_button(buffer, &btn_save, mx, my) && clicked {
        //     let _ = std::fs::write("keysAction.txt", game.map_index.to_string());
        //     return AppState::KeyChangeScreen;
        // }

        let btn_menu = ButtonDef {
            x: 405,
            y: 405,
            w: 320,
            h: 30,
            text: "[ESC] MAIN MENU".to_string(),
        }; 

        
        let action_btn: [(&ButtonDef, GameAction); 19] = [
            (&btn_forwards, GameAction::Forwards),
            (&btn_left, GameAction::Left),
            (&btn_backwards, GameAction::Backwards),
            (&btn_right, GameAction::Right),
            (&btn_sprint, GameAction::Sprint),
            (&btn_jump, GameAction::Jump),
            (&btn_crouch, GameAction::Crouch),
            (&btn_rocket_jump, GameAction::RocketJump),
            (&btn_interact, GameAction::Interact),
            (&btn_toggle_fine_aiming, GameAction::ToggleFineAiming),
            (&btn_move_crosshair_up, GameAction::CrosshairUp),
            (&btn_turn_left, GameAction::TurnLeft),
            (&btn_move_crosshair_down, GameAction::CrosshairDown),
            (&btn_turn_right, GameAction::TurnRight),
            (&btn_fire_bullet, GameAction::FireBullet),
            (&btn_fire_arrow, GameAction::FireArrow),
            (&btn_enter_godmode, GameAction::EnterGodmode),
            (&btn_lock_enemies, GameAction::LockEnemies),
            (&btn_pause_game, GameAction::PauseGame),
        ];
                                          
        use crate::keys::key_assigned::change_key_and_save;      

        //Action Button Funktionen
        for (button, game_action) in action_btn {

            if self.draw_flat_button(buffer, &button, mx, my) && clicked {

                buffer.fill(COL_BG_KEYCHANGE);

                Self::draw_text_centered(
                    buffer, SCREEN_WIDTH, SCREEN_HEIGHT,
                    "CHANGE KEY BINDINGS",
                    25,
                    4,
                    COL_TITLE_BLUE,
                );
                Self::draw_text_centered(
                    buffer, SCREEN_WIDTH, SCREEN_HEIGHT,
                    "Using one key for multible game actions isn't possible. Return to Main Menu to save changes.",
                    75,
                    1,
                    COL_TITLE_BLUE,
                );
                Self::draw_text_centered(
                    buffer, SCREEN_WIDTH, SCREEN_HEIGHT,
                    "Press the key you want to bind the action to.",
                    90,
                    2,
                    COL_TITLE_RED,
                );
                

                if self.draw_flat_button(buffer, &button, mx, my){

                    change_key_and_save(
                        &game_action, 
                        key_map, 
                        window, 
                        audio,
                    );
                }
            }
        }
        
        if self.draw_flat_button(buffer, &btn_menu, mx, my) && clicked || click_esc {
            // on menu close
            println!("{:?}", key_map);
            let result = save_key_bindings_in_file(&key_map);
            println!("{:?}", result);
            println!("{:?}", key_map);
            return AppState::StartScreen;
        }

        return AppState::KeyChangeScreen;
    }


    /// Fetches mouse coordinates and performs edge detection (debouncing) for left-clicks.
    fn get_mouse_state(&mut self, window: &Window) -> (usize, usize, bool) {
        // Clamp prevents out-of-bounds coordinates if the mouse leaves the window frame
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let is_mouse_down = window.get_mouse_down(MouseButton::Left);

        let clicked = is_mouse_down && !self.was_mouse_down;
        self.was_mouse_down = is_mouse_down;

        (mouse_pos.0 as usize, mouse_pos.1 as usize, clicked)
    }

    /// Draws a rectangular button and its text onto the buffer.
    /// Returns `true` if the mouse cursor is currently hovering over it.
    fn draw_flat_button(&self, buffer: &mut [u32], btn: &ButtonDef, mx: usize, my: usize) -> bool {
        // Simple check for hover detection
        let hover = mx >= btn.x && mx <= (btn.x + btn.w) && my >= btn.y && my <= (btn.y + btn.h);
        let bg_color = if hover { COL_BTN_HOVER } else { COL_BTN_NORMAL };

        // Rasterize the rectangle into the 1D pixel buffer
        for row in btn.y..(btn.y + btn.h) {
            for col in btn.x..(btn.x + btn.w) {
                // Safety check to prevent panic from out-of-bounds memory writes
                if row < SCREEN_HEIGHT && col < SCREEN_WIDTH {
                    buffer[row * SCREEN_WIDTH + col] = bg_color;
                }
            }
        }

        // Center the text horizontally and vertically inside the button bounds
        let text_width = btn.text.len() * 8 * 2;
        let text_x = btn.x + btn.w.saturating_sub(text_width) / 2;
        let text_y = btn.y + btn.h.saturating_sub(16) / 2;

        Self::draw_text(
            buffer,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &btn.text,
            text_x,
            text_y,
            2,
            COL_TEXT_WHITE,
        );

        hover
    }

    /// Helper to align a given text string exactly in the horizontal center of the screen.
    fn draw_text_centered(
        buffer: &mut [u32],
        screen_width: usize,
        screen_height: usize,
        text: &str,
        y: usize,
        scale: usize,
        color: u32,
    ) {
        // Each character in the font is 8 pixels wide, multiplied by the scaling factor
        let text_width = text.len() * 8 * scale;
        let x = screen_width.saturating_sub(text_width) / 2;

        Self::draw_text(
            buffer,
            screen_width,
            screen_height,
            text,
            x,
            y,
            scale,
            color,
        );
    }

    /// Software text rasterizer using the `font8x8` library.
    /// It translates strings into raw pixel data and pushes them to the 1D buffer.
    #[allow(clippy::too_many_arguments)]
    fn draw_text(
        buffer: &mut [u32],
        screen_width: usize,
        screen_height: usize,
        text: &str,
        start_x: usize,
        start_y: usize,
        scale: usize,
        color: u32,
    ) {
        let mut cursor_x = start_x;

        for c in text.chars() {
            if let Some(glyph) = BASIC_FONTS.get(c) {
                #[allow(clippy::needless_range_loop)]
                for y in 0..8 {
                    for x in 0..8 {
                        // Bitwise check: if the specific bit in the glyph byte is a 1, a pixel needs to be drawn
                        if (glyph[y] & (1 << x)) != 0 {
                            // Nearest-neighbor scaling (draws larger 2D blocks for each active pixel based on the `scale` factor)
                            for dy in 0..scale {
                                for dx in 0..scale {
                                    let px = cursor_x + x * scale + dx;
                                    let py = start_y + y * scale + dy;

                                    if px < screen_width && py < screen_height {
                                        // Flatten 2D coordinate into a 1D array index
                                        buffer[py * screen_width + px] = color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Advance the cursor by the width of the character to prepare for the next letter
            cursor_x += 8 * scale;
        }
    }
}
