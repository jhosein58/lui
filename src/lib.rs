pub mod graphics;
pub mod widgets;
pub mod monitoring;

pub use graphics::screen::Screen;
pub use graphics::color::Color;
pub use graphics::gbuf::GBuf;

pub use widgets::Widget;
pub use widgets::props::style::Style;
pub use widgets::props::size::Size;
pub use widgets::props::size::Size::*;
pub use widgets::body::Body;
pub use widgets::nil::Nil;
pub use widgets::rectangle::Rectangle;

pub use monitoring::fps::Fps;