pub mod menu_item;

pub use menu_item::MenuItem;

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Text},
    Drawable,
};
use embedded_layout::prelude::*;
use u8g2_fonts::U8g2TextStyle;

/// Marker trait necessary to avoid a "conflicting implementations" error.
pub trait Marker {}

pub trait MenuListItem<R>: Marker + View {
    /// Returns the value of the selected item, without interacting with it.
    fn value_of(&self) -> R;

    fn interact(&mut self) -> R;

    fn set_style(&mut self, text_style: &U8g2TextStyle<BinaryColor>);

    /// Returns whether the list item is selectable.
    ///
    /// If this returns false, the list item will not be interactable and user navigation will skip
    /// over it.
    fn selectable(&self) -> bool {
        true
    }

    fn draw_styled<D>(
        &self,
        text_style: &U8g2TextStyle<BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
}

/// Helper struct to draw a menu line that has a title and some additional marker.
pub struct MenuLine {
    bounds: Rectangle,
    value_width: u32,
}

impl MenuLine {
    pub fn new(value_width: u32, text_style: &U8g2TextStyle<BinaryColor>) -> Self {
        MenuLine {
            bounds: Rectangle::new(Point::zero(), Size::new(1, text_style.line_height() - 1)),
            value_width,
        }
    }

    pub fn empty() -> Self {
        MenuLine {
            bounds: Rectangle::new(Point::zero(), Size::new(1, 0)),
            value_width: 0,
        }
    }

    pub fn draw_styled<D>(
        &self,
        title: &str,
        value_text: &str,
        text_style: &U8g2TextStyle<BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        if self.bounds.intersection(&display_area).size.height == 0 {
            return Ok(());
        }
        let mut text_bounds = Rectangle::new(
            self.bounds.top_left,
            Size::new(display_area.size.width, self.bounds.size.height),
        );

        // draw right value text
        Text::new(value_text, Point::zero(), text_style)
            .align_to(&text_bounds, horizontal::Right, vertical::Center)
            .draw(display)?;

        // draw left title text
        text_bounds.size.width -= self.value_width;
        Text::new(title, Point::zero(), text_style)
            .align_to(&text_bounds, horizontal::Left, vertical::Center)
            .draw(display)?;

        Ok(())
    }
}

impl View for MenuLine {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}
