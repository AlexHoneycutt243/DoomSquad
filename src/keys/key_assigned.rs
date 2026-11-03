use std::collections::HashMap;
use std::fs;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use ron::from_str;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use crate::GameAction;
use crate::audio::audio_handler::Audio;
use crate::menu::menu_handler::{AppState};
use std::fs::File;
use std::io::{Write};


pub fn load_key_bindings_file_parse() -> Result<HashMap<Key, GameAction>, Box<dyn std::error::Error>>{
    let file = fs::read_to_string("key_assigned_file.ron")?;
    let key_map: HashMap<ZwischenKey, GameAction> = from_str(&file)?;
    let key_map_translated: Vec<(Key, GameAction)> = key_map
        .into_iter()
        .map(|(zwischenkey, value)| {
            (from_zwischenkey_to_key(zwischenkey),value)
        })
        .collect();
    let key_map_new: HashMap<Key, GameAction> =
        HashMap::from_iter(key_map_translated);
    Ok(key_map_new)
}

pub fn save_key_bindings_in_file(key_map: &HashMap<Key, GameAction>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("key_assigned_file.ron")?;
    let key_map_translated: Vec<(ZwischenKey, GameAction)> = key_map.into_iter().map(|(key, value)| {
        (from_key_to_zwischenkey(*key), *value)
    })
    .collect();
    let key_map_new: HashMap<ZwischenKey, GameAction> = HashMap::from_iter(key_map_translated);
    let ron_file = to_string_pretty(&key_map_new, PrettyConfig::default())?;
    file.write(ron_file.as_bytes())?;
    println!("Rocky hate Mark");
    Ok(())
}

pub fn change_key_and_save(
    game_action_clicked: &GameAction,
    key_map: &mut HashMap<Key, GameAction>, 
    window: &mut Window, 
    audio: &mut Audio,
) -> AppState {   
    loop {
        window.update();

        if let Some(key) = window
            .get_keys_pressed(KeyRepeat::No)
            .first()
        {
            let new_key = *key;

            println!("{:?}", key_map);

            // Prüfen, ob die Taste bereits einer anderen Aktion gehört
            let key_already_used = key_map.iter().any(|(key, action)| {
                *key == new_key && action != game_action_clicked
            });

            if key_already_used {
                // Taste ist bereits vergeben:
                // Keine Änderung durchführen.
                continue;
            }

            // Alte Taste dieser Aktion entfernen
            key_map.retain(|_, action| action != game_action_clicked);
            
            key_map.insert(new_key, *game_action_clicked);

            println!("{:?}", key_map);

            audio.play_sfx("slotmachine", 1.0);
            return AppState::KeyChangeScreen;
        }
    }
}
    


// pub fn is_action_key_pressed (
//         myGameAction: GameAction, 
//         key_map: &HashMap<Key, GameAction>,
//         window: &mut Window,
//     ) -> bool {
//     let index_GameAction = myGameAction as usize;
//     if let Some((_, key_at_index)) = &keys_current_array[index_GameAction] {
//         window.is_key_pressed(key_at_index, KeyRepeat::No)
//     } else {false}
// }

// pub fn is_action_key_down (
//     myGameAction: GameAction, 
//     keys_current_array: &[Option<(String, Key)>; 19],
//     window: &mut Window,
//     ) -> bool {
//     let index_GameAction = myGameAction as usize;
//     if let Some((_, key_at_index)) = &keys_current_array[index_GameAction] {
//         window.is_key_down (key_at_index)
//     } else {false} 
// }


fn from_zwischenkey_to_key(my_zwischenkey: ZwischenKey) -> Key {
    match my_zwischenkey {
        ZwischenKey::Key1 => Key::Key1,
        ZwischenKey::Key2 => Key::Key2,
        ZwischenKey::Key3 => Key::Key3,
        ZwischenKey::Key4 => Key::Key4,
        ZwischenKey::Key5 => Key::Key5,
        ZwischenKey::Key6 => Key::Key6,
        ZwischenKey::Key0 => Key::Key0,
        ZwischenKey::Key7 => Key::Key7,
        ZwischenKey::Key8 => Key::Key8,
        ZwischenKey::Key9 => Key::Key9,

        ZwischenKey::A => Key::A,
        ZwischenKey::B => Key::B,
        ZwischenKey::C => Key::C,
        ZwischenKey::D => Key::D,
        ZwischenKey::E => Key::E,
        ZwischenKey::F => Key::F,
        ZwischenKey::G => Key::G,
        ZwischenKey::H => Key::H,
        ZwischenKey::I => Key::I,
        ZwischenKey::J => Key::J,
        ZwischenKey::K => Key::K,
        ZwischenKey::L => Key::L,
        ZwischenKey::M => Key::M,
        ZwischenKey::N => Key::N,
        ZwischenKey::O => Key::O,
        ZwischenKey::P => Key::P,
        ZwischenKey::Q => Key::Q,
        ZwischenKey::R => Key::R,
        ZwischenKey::S => Key::S,
        ZwischenKey::T => Key::T,
        ZwischenKey::U => Key::U,
        ZwischenKey::V => Key::V,
        ZwischenKey::W => Key::W,
        ZwischenKey::X => Key::X,
        ZwischenKey::Y => Key::Y,
        ZwischenKey::Z => Key::Z,

        ZwischenKey::F1 => Key::F1,
        ZwischenKey::F2 => Key::F2,
        ZwischenKey::F3 => Key::F3,
        ZwischenKey::F4 => Key::F4,
        ZwischenKey::F5 => Key::F5,
        ZwischenKey::F6 => Key::F6,
        ZwischenKey::F7 => Key::F7,
        ZwischenKey::F8 => Key::F8,
        ZwischenKey::F9 => Key::F9,
        ZwischenKey::F10 => Key::F10,
        ZwischenKey::F11 => Key::F11,
        ZwischenKey::F12 => Key::F12,
        ZwischenKey::F13 => Key::F13,
        ZwischenKey::F14 => Key::F14,
        ZwischenKey::F15 => Key::F15,

        ZwischenKey::Down => Key::Down,
        ZwischenKey::Left => Key::Left,
        ZwischenKey::Right => Key::Right,
        ZwischenKey::Up => Key::Up,
        ZwischenKey::Apostrophe => Key::Apostrophe,
        ZwischenKey::Backquote => Key::Backquote,
        ZwischenKey::Backslash => Key::Backslash,
        ZwischenKey::Comma => Key::Comma,
        ZwischenKey::Equal => Key::Equal,
        ZwischenKey::LeftBracket => Key::LeftBracket,
        ZwischenKey::Minus => Key::Minus,
        ZwischenKey::Period => Key::Period,
        ZwischenKey::RightBracket => Key::RightBracket,
        ZwischenKey::Semicolon => Key::Semicolon,
        ZwischenKey::Slash => Key::Slash,
        ZwischenKey::Backspace => Key::Backspace,
        ZwischenKey::Delete => Key::Delete,
        ZwischenKey::End => Key::End,
        ZwischenKey::Enter => Key::Enter,
        ZwischenKey::Escape => Key::Escape,
        ZwischenKey::Home => Key::Home,
        ZwischenKey::Insert => Key::Insert,
        ZwischenKey::Menu => Key::Menu,
        ZwischenKey::PageDown => Key::PageDown,
        ZwischenKey::PageUp => Key::PageUp,
        ZwischenKey::Pause => Key::Pause,
        ZwischenKey::Space => Key::Space,
        ZwischenKey::Tab => Key::Tab,
        ZwischenKey::NumLock => Key::NumLock,
        ZwischenKey::CapsLock => Key::CapsLock,
        ZwischenKey::ScrollLock => Key::ScrollLock,
        ZwischenKey::LeftShift => Key::LeftShift,
        ZwischenKey::RightShift => Key::RightShift,
        ZwischenKey::LeftCtrl => Key::LeftCtrl,
        ZwischenKey::RightCtrl => Key::RightCtrl,
        ZwischenKey::NumPad0 => Key::NumPad0,
        ZwischenKey::NumPad1 => Key::NumPad1,
        ZwischenKey::NumPad2 => Key::NumPad2,
        ZwischenKey::NumPad3 => Key::NumPad3,
        ZwischenKey::NumPad4 => Key::NumPad4,
        ZwischenKey::NumPad5 => Key::NumPad5,
        ZwischenKey::NumPad6 => Key::NumPad6,
        ZwischenKey::NumPad7 => Key::NumPad7,
        ZwischenKey::NumPad8 => Key::NumPad8,
        ZwischenKey::NumPad9 => Key::NumPad9,
        ZwischenKey::NumPadDot => Key::NumPadDot,
        ZwischenKey::NumPadSlash => Key::NumPadSlash,
        ZwischenKey::NumPadAsterisk => Key::NumPadAsterisk,
        ZwischenKey::NumPadMinus => Key::NumPadMinus,
        ZwischenKey::NumPadPlus => Key::NumPadPlus,
        ZwischenKey::NumPadEnter => Key::NumPadEnter,
        ZwischenKey::LeftAlt => Key::LeftAlt,
        ZwischenKey::RightAlt => Key::RightAlt,
        ZwischenKey::LeftSuper => Key::LeftSuper,
        ZwischenKey::RightSuper => Key::RightSuper,
        ZwischenKey::Unknown => Key::Unknown,
        ZwischenKey::Count => Key::Count,
    }
}

fn from_key_to_zwischenkey(key: Key) -> ZwischenKey {
    match key {
        Key::Key1 => ZwischenKey::Key1,
        Key::Key2 => ZwischenKey::Key2,
        Key::Key3 => ZwischenKey::Key3,
        Key::Key4 => ZwischenKey::Key4,
        Key::Key5 => ZwischenKey::Key5,
        Key::Key6 => ZwischenKey::Key6,
        Key::Key0 => ZwischenKey::Key0,
        Key::Key7 => ZwischenKey::Key7,
        Key::Key8 => ZwischenKey::Key8,
        Key::Key9 => ZwischenKey::Key9,

        Key::A => ZwischenKey::A,
        Key::B => ZwischenKey::B,
        Key::C => ZwischenKey::C,
        Key::D => ZwischenKey::D,
        Key::E => ZwischenKey::E,
        Key::F => ZwischenKey::F,
        Key::G => ZwischenKey::G,
        Key::H => ZwischenKey::H,
        Key::I => ZwischenKey::I,
        Key::J => ZwischenKey::J,
        Key::K => ZwischenKey::K,
        Key::L => ZwischenKey::L,
        Key::M => ZwischenKey::M,
        Key::N => ZwischenKey::N,
        Key::O => ZwischenKey::O,
        Key::P => ZwischenKey::P,
        Key::Q => ZwischenKey::Q,
        Key::R => ZwischenKey::R,
        Key::S => ZwischenKey::S,
        Key::T => ZwischenKey::T,
        Key::U => ZwischenKey::U,
        Key::V => ZwischenKey::V,
        Key::W => ZwischenKey::W,
        Key::X => ZwischenKey::X,
        Key::Y => ZwischenKey::Y,
        Key::Z => ZwischenKey::Z,

        Key::F1 => ZwischenKey::F1,
        Key::F2 => ZwischenKey::F2,
        Key::F3 => ZwischenKey::F3,
        Key::F4 => ZwischenKey::F4,
        Key::F5 => ZwischenKey::F5,
        Key::F6 => ZwischenKey::F6,
        Key::F7 => ZwischenKey::F7,
        Key::F8 => ZwischenKey::F8,
        Key::F9 => ZwischenKey::F9,
        Key::F10 => ZwischenKey::F10,
        Key::F11 => ZwischenKey::F11,
        Key::F12 => ZwischenKey::F12,
        Key::F13 => ZwischenKey::F13,
        Key::F14 => ZwischenKey::F14,
        Key::F15 => ZwischenKey::F15,

        Key::Down => ZwischenKey::Down,
        Key::Left => ZwischenKey::Left,
        Key::Right => ZwischenKey::Right,
        Key::Up => ZwischenKey::Up,
        Key::Apostrophe => ZwischenKey::Apostrophe,
        Key::Backquote => ZwischenKey::Backquote,
        Key::Backslash => ZwischenKey::Backslash,
        Key::Comma => ZwischenKey::Comma,
        Key::Equal => ZwischenKey::Equal,
        Key::LeftBracket => ZwischenKey::LeftBracket,
        Key::Minus => ZwischenKey::Minus,
        Key::Period => ZwischenKey::Period,
        Key::RightBracket => ZwischenKey::RightBracket,
        Key::Semicolon => ZwischenKey::Semicolon,
        Key::Slash => ZwischenKey::Slash,
        Key::Backspace => ZwischenKey::Backspace,
        Key::Delete => ZwischenKey::Delete,
        Key::End => ZwischenKey::End,
        Key::Enter => ZwischenKey::Enter,
        Key::Escape => ZwischenKey::Escape,
        Key::Home => ZwischenKey::Home,
        Key::Insert => ZwischenKey::Insert,
        Key::Menu => ZwischenKey::Menu,
        Key::PageDown => ZwischenKey::PageDown,
        Key::PageUp => ZwischenKey::PageUp,
        Key::Pause => ZwischenKey::Pause,
        Key::Space => ZwischenKey::Space,
        Key::Tab => ZwischenKey::Tab,
        Key::NumLock => ZwischenKey::NumLock,
        Key::CapsLock => ZwischenKey::CapsLock,
        Key::ScrollLock => ZwischenKey::ScrollLock,
        Key::LeftShift => ZwischenKey::LeftShift,
        Key::RightShift => ZwischenKey::RightShift,
        Key::LeftCtrl => ZwischenKey::LeftCtrl,
        Key::RightCtrl => ZwischenKey::RightCtrl,
        Key::NumPad0 => ZwischenKey::NumPad0,
        Key::NumPad1 => ZwischenKey::NumPad1,
        Key::NumPad2 => ZwischenKey::NumPad2,
        Key::NumPad3 => ZwischenKey::NumPad3,
        Key::NumPad4 => ZwischenKey::NumPad4,
        Key::NumPad5 => ZwischenKey::NumPad5,
        Key::NumPad6 => ZwischenKey::NumPad6,
        Key::NumPad7 => ZwischenKey::NumPad7,
        Key::NumPad8 => ZwischenKey::NumPad8,
        Key::NumPad9 => ZwischenKey::NumPad9,
        Key::NumPadDot => ZwischenKey::NumPadDot,
        Key::NumPadSlash => ZwischenKey::NumPadSlash,
        Key::NumPadAsterisk => ZwischenKey::NumPadAsterisk,
        Key::NumPadMinus => ZwischenKey::NumPadMinus,
        Key::NumPadPlus => ZwischenKey::NumPadPlus,
        Key::NumPadEnter => ZwischenKey::NumPadEnter,
        Key::LeftAlt => ZwischenKey::LeftAlt,
        Key::RightAlt => ZwischenKey::RightAlt,
        Key::LeftSuper => ZwischenKey::LeftSuper,
        Key::RightSuper => ZwischenKey::RightSuper,
        Key::Unknown => ZwischenKey::Unknown,
        Key::Count => ZwischenKey::Count,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZwischenKey {
    Key0 = 0,
    Key1 = 1,
    Key2 = 2,
    Key3 = 3,
    Key4 = 4,
    Key5 = 5,
    Key6 = 6,
    Key7 = 7,
    Key8 = 8,
    Key9 = 9,

    A = 10,
    B = 11,
    C = 12,
    D = 13,
    E = 14,
    F = 15,
    G = 16,
    H = 17,
    I = 18,
    J = 19,
    K = 20,
    L = 21,
    M = 22,
    N = 23,
    O = 24,
    P = 25,
    Q = 26,
    R = 27,
    S = 28,
    T = 29,
    U = 30,
    V = 31,
    W = 32,
    X = 33,
    Y = 34,
    Z = 35,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,

    Down,
    Left,
    Right,
    Up,
    Apostrophe,
    Backquote,

    Backslash,
    Comma,
    Equal,
    LeftBracket,
    Minus,
    Period,
    RightBracket,
    Semicolon,

    Slash,
    Backspace,
    Delete,
    End,
    Enter,

    Escape,

    Home,
    Insert,
    Menu,

    PageDown,
    PageUp,

    Pause,
    Space,
    Tab,
    NumLock,
    CapsLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,

    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadDot,
    NumPadSlash,
    NumPadAsterisk,
    NumPadMinus,
    NumPadPlus,
    NumPadEnter,

    LeftAlt,
    RightAlt,

    LeftSuper,
    RightSuper,

    /// Used when an Unknown key has been pressed
    Unknown,

    Count = 107,
}
