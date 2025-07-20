pub mod graphics;
pub mod widgets;
pub mod monitoring;

pub use graphics::screen::Screen;
pub use graphics::color::Color;
pub use graphics::gbuf::GBuf;
pub use graphics::font::Font;

pub use graphics::processors::Processor;
pub use graphics::processors::border_radius::BorderRadius;

pub use widgets::Widget;
pub use widgets::DefStyle;
pub use widgets::props::style::Style;
pub use widgets::props::size::Size;
pub use widgets::props::pos::PosVal;
pub use widgets::props::pos::Position;
pub use widgets::props::dir::Dir;
pub use widgets::props::dir::DirVal;
pub use widgets::body::Body;
pub use widgets::nil::Nil;
pub use widgets::rectangle::Rectangle;
pub use widgets::container::Container;
pub use widgets::rawbuf::RawBuf;
pub use widgets::text::Text;
pub use widgets::button::Button;

pub use monitoring::fps::Fps;