fn main() {
	if cfg!(target_os = "windows") {
		let mut res = winres::WindowsResource::new();
		res.set_icon("resource/state3.ico");
		res.set_icon_with_id("resource/state0.ico", "state0-icon");
		res.set_icon_with_id("resource/state1.ico", "state1-icon");
		res.set_icon_with_id("resource/state2.ico", "state2-icon");
		res.set_icon_with_id("resource/state3.ico", "state3-icon");
		res.compile().expect("compiling resource file");
	}
}