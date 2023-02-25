use crate::{linux::wayland_screenshot::wayland_screenshot, DisplayInfo};

/// modified to return a bgra buffer, not a PNG
pub fn wayland_capture_screen_area(
    display_info: &DisplayInfo,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Option<(Vec<u8>, bool)> {
    let area_x = (((x + display_info.x) as f32) * display_info.scale_factor) as i32;
    let area_y = (((y + display_info.y) as f32) * display_info.scale_factor) as i32;
    let area_width = (width as f32) * display_info.scale_factor;
    let area_height = (height as f32) * display_info.scale_factor;

    let result = wayland_screenshot(area_x, area_y, area_width as i32, area_height as i32)?;
    Some(result)
}
