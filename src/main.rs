extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use core::panic;

use nwd::NwgUi;
use nwg::NativeUi;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VK_CAPITAL, VK_NUMLOCK};

#[derive(Default, NwgUi)]
pub struct LockIndicator {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("STATE0ICON"))]
    state0icon: nwg::Icon,
    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("STATE1ICON"))]
    state1icon: nwg::Icon,
    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("STATE2ICON"))]
    state2icon: nwg::Icon,
    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("STATE3ICON"))]
    state3icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.state0icon), tip: Some("Caps/Num Lock Indicator"))]
    #[nwg_events(MousePressLeftUp: [LockIndicator::show_menu], OnContextMenu: [LockIndicator::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [LockIndicator::exit])]
    tray_item: nwg::MenuItem,
}

impl LockIndicator {
    fn change_icon(&self, state: u8) {
        match state {
            0 => self.tray.set_icon(&self.state0icon),
            1 => self.tray.set_icon(&self.state1icon),
            2 => self.tray.set_icon(&self.state2icon),
            3 => self.tray.set_icon(&self.state3icon),
            _ => panic!("state out of range"),
        }
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = LockIndicator::build_ui(Default::default()).expect("Failed to build UI");

    let mut last_state = 0;
    nwg::dispatch_thread_events_with_callback(move || {
        let mut state = 0;

        unsafe {
            if GetKeyState(VK_CAPITAL.0.into()) == 1 { state += 1 };
            if GetKeyState(VK_NUMLOCK.0.into()) == 1 { state += 2 };
        }

        if last_state != state {
            ui.change_icon(state);
            last_state = state;
        }
    });
}