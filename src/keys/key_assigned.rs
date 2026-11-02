use std::fs;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use crate::menu::menu_handler::ButtonDef;
use std::path::Path;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};



pub fn look_for_key_file_or_create(/*keys_current_array: &mut [Option<(String, Key)>; 19]*/) {
//     let pfad = Path::new("key.assigned.json");
//     match pfad.try_exists() {
//         Ok(true) => {read_file_into_array(&mut [Option<(String, Key)>; 19]);
//         },
//         Ok(false) => {create_file();
//                       read_file_into_array(&mut [Option<(String, Key)>; 19]);
//         },
//         Err(e) => println!("Could not check path: {}", e),
//     }

//     fn create_file() -> std::io::Result<()> {
//         let mut datei = File::create("key.assigned.json")?;
//         writeln!(datei, "btn_forwards,Key::W")?;
//         writeln!(datei, "btn_left,Key::A")?;
//         writeln!(datei, "btn_backwards,Key::S")?;
//         writeln!(datei, "btn_right,Key::D")?;
//         writeln!(datei, "btn_sprint,Key::LeftShift")?;
//         writeln!(datei, "btn_jump,Key::Space")?;
//         writeln!(datei, "btn_crouch,Key::C")?;
//         writeln!(datei, "btn_rocket_jump,Key::R")?;
//         writeln!(datei, "btn_interact,Key::F")?;
//         writeln!(datei, "btn_toggle_fine_aiming,Key::E")?;
//         writeln!(datei, "btn_move_crosshair_up,Key::Up")?;
//         writeln!(datei, "btn_turn_left,Key::Left")?;
//         writeln!(datei, "btn_move_crosshair_down,Key::Down")?;
//         writeln!(datei, "btn_turn_right,Key::Right")?;
//         writeln!(datei, "btn_fire_bullet,Key::RightCtrl")?;
//         writeln!(datei, "btn_fire_arrow,Key::RightShift")?;
//         writeln!(datei, "btn_enter_godmode,Key::G")?;
//         writeln!(datei, "btn_lock_enemies,Key::L")?;
//         writeln!(datei, "btn_exit_game,Key::Escape")?;
//         Ok(())
//     }
    
//     fn read_file_into_array(keys_current_array: &mut [Option<(String, Key)>; 19]) -> io::Result<()> {
//         let pfad = Path::new("key.assigned.json");
//         // 1. Datei öffnen
    
//         let file = File::open(&path)?;
    
//         // 2. Mit BufReader puffern, um zeilenweise lesen zu können
//         let reader = BufReader::new(file);
        
//         // Vektor, um die fertigen Tupel zu speichern
//         let mut resultate: Vec<(String, u32)> = Vec::new();

//         // 3. Zeilenweise durchlaufen
//         for line in reader.lines() {
//             let line = line?; // Fehler beim Lesen der Zeile abfangen
            
//             // Zeile am Komma trennen und Whitespaces entfernen
//             let teile: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                
//             // Sicherstellen, dass wir genau zwei Elemente haben
//             if teile.len() == 2 {
//                 let str_action_btn = teile[0].to_string();
//                     // Zahl parsen und Fehler abfangen (z.B. wenn es keine Zahl ist)
//                 if let Ok(key_assigned_action_key) = teile[1].parse::<Key>() {
//                         // Als Tupel in das array packen
//                         ((str_action_btn, key_assigned_action_key));
//                 }
//             }
//         }

//         // Ergebnis ausgeben
//         for tupel in &resultate {
//             println!("Name: {}, Alter: {}", tupel.0, tupel.1);
//         }

//         Ok(())
//     }

//     Ok(())
 }


pub fn change_key_and_save(
// //         clicked_action_btn: &ButtonDef,
// //         keys_current_array: &mut [Option<(String, Key)>; 19], 
// //         window: &mut Window, 
// //         audio: &mut Audio,
// //     ) -> AppState {
)  {

// //     // ButtonDef in einen String umwandeln
// //     let clicked_action_name = clicked_action_btn.to_string();
// //     // Warten, bis irgendeine Taste gedrückt wird
// //     loop {
// //         window.update();

// //         if let Some(key) = window
// //             .get_keys_pressed(KeyRepeat::No)
// //             .first()
// //         {
// //             // Key::Key4 gedrückt?
// //             if *key == Key::Key4 {
// //             return;
// //             }
// //             // Durch das Array iterieren
// //             for entry in keys_current_array.iter_mut() {
// //                 if let Some((button_name, gespeicherter_key)) = entry {

// //                     // Prüfen, ob der Buttonname übereinstimmt
// //                     if *button_name == clicked_action_name {
// //                         // Neuen Key in die zweite Stelle des Tupels schreiben
// //                         *gespeicherter_key = *key;

// //                         // Key wurde gespeichert -> Funktion beenden
// //                         // success sound
// //                         audio.play_sfx("slotmachine", 1.0);
// //                         let mut rng = rand::rng();
// //                         return AppState::KeyChangeScreen;
// //                     }
// //                 }
// //             }
// //         }
// //     }
}
// // //ist key == key:: 4? dann return AppState::KeyChangeScreen;
// // //ändere key im array wobei der erste eintrag array immer ein Buttonname als String ist, 
// // //ändere key in datei
    


// pub fn is_action_key_pressed (
//         myGameAction: GameAction, 
//         keys_current_array: &[Option<(String, Key)>; 19],
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