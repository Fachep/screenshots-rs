mod wayland;
mod wayland_screenshot;
mod xorg;

use crate::DisplayInfo;

use std::env::var_os;
use wayland::wayland_capture_screen_area;
use xorg::xorg_capture_screen_area;

fn wayland_detect() -> bool {
    let xdg_session_type = var_os("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let wayland_display = var_os("WAYLAND_DISPLAY")
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    xdg_session_type.eq("wayland") || wayland_display.to_lowercase().contains("wayland")
}

/// return raw BGRA/RGBA buffer. if re.1 is true, you receive BGRA buffer.
pub fn capture_screen_area(
    display_info: &DisplayInfo,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Option<(Vec<u8>, bool)> {
    if wayland_detect() {
        wayland_capture_screen_area(display_info, x, y, width, height)
    } else {
        xorg_capture_screen_area(display_info, x, y, width, height)
    }
}
