// for capturing the keyboard combos to trigger mouse capture
use rdev::{listen, Event,Key, EventType};

fn keycap() {
     let mut ordered_keys= Vec::new();

    // Start listening for key events
    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            // When a key is pressed, add it to the set of pressed keys
            EventType::KeyPress(key)=> {
                if ordered_keys.contains(&key) == false {
                    ordered_keys.push(key);
                }
                
            }
            // When a key is released, remove it from the set of pressed keys
            EventType::KeyRelease(key) => {
                if ordered_keys.contains(&key) == true {
                    // removes all instances of key, if there are duplicates
                    ordered_keys.retain( |&x| x!= key);
                }  
            }
            _ => {}
        }

        // Check if the desired combination of keys is pressed

        if ordered_keys.contains(&Key::ControlLeft) && ordered_keys.contains(&Key::ShiftLeft) && ordered_keys.contains(&Key::KeyQ){
            if ordered_keys.iter().position(|&x| x == Key::ControlLeft) <  ordered_keys.iter().position(|&x| x == Key::ShiftLeft) < ordered_keys.iter().position(|&x| x == Key::KeyQ) {
                //trigger mouse capture with UI
            }

            
        }
    }) {
        println!("Error occurred: {:?}", error);
    }
}