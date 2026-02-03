mod footer;
mod layout;
mod puzzle;
mod region;
mod rules;

use std::fmt::Display;

pub use footer::*;
pub use layout::*;
pub use puzzle::*;
pub use region::*;
pub use rules::*;

use ratatui::{buffer::Buffer, layout::Position as AppPosition, style::Style};

pub fn safe_draw_str<T>(buf: &mut Buffer, pos: AppPosition, content: T, style: Style)
where
    T: AsRef<str> + Display,
{
    let right = pos.x + content.as_ref().len().saturating_sub(1) as u16;
    let final_pos = AppPosition::new(right, pos.y);

    if !buf.area.contains(final_pos) {
        tracing::warn!(
            "Not writing {content} at {pos}-{final_pos}, falls outside the area {:?}",
            buf.area
        );
        return;
    }

    buf.set_string(pos.x, pos.y, content, style);
}
