#![windows_subsystem = "windows"]

use std::sync::mpsc;
use std::time::Duration;
use tray_item::TrayItem;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_CAPITAL, VK_NUMLOCK};

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut tray = TrayItem::new("Caps/Num Lock Indicator", "state0-icon").expect("creating tray");
    tray.add_menu_item("Quit", move || {
        tx.send(true).expect("sending channel message");
    }).expect("adding menu item");
    
    let mut last_state = 0;

    loop {
        let mut state = 0;

        unsafe {
            if GetKeyState(VK_CAPITAL.0.into()) == 1 { state += 1 };
            if GetKeyState(VK_NUMLOCK.0.into()) == 1 { state += 2 };
        }

        if last_state != state {
            let mut string_icon = String::from("state");
            string_icon.push_str(&state.to_string());
            string_icon.push_str("-icon");
            tray.set_icon(&string_icon).expect("switching icon");
            last_state = state
        }

        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }
}
