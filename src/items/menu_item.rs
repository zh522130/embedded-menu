use crate::items::{Marker, MenuLine, MenuListItem};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
};
use embedded_layout::View;
use u8g2_fonts::U8g2TextStyle;

pub trait SelectValue: Sized + Clone + PartialEq {
    /// Transforms the value on interaction
    fn next(&mut self) {}

    /// Returns a displayable marker for the value
    fn marker(&self) -> &str;
}

impl SelectValue for bool {
    fn next(&mut self) {
        *self = !*self;
    }

    fn marker(&self) -> &str {
        match *self {
            // true => "O",
            // false => "O\r+\r#", // this only works for certain small fonts, unfortunately
            false => "[N]",
            true => "[Y]",
        }
    }
}

impl SelectValue for &str {
    fn marker(&self) -> &str {
        self
    }
}

impl SelectValue for () {
    fn marker(&self) -> &str {
        ""
    }
}

pub struct MenuItem<T, R, S, const SELECTABLE: bool>
where
    T: AsRef<str>,
    S: SelectValue,
{
    title_text: T,
    convert: fn(S) -> R,
    value: S,
    line: MenuLine,
}

impl<T, S> MenuItem<T, (), S, true>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn new(title_text: T, value: S) -> Self {
        Self {
            title_text,
            value,
            convert: |_| (),
            line: MenuLine::empty(),
        }
    }
}

impl<T, R, S, const SELECTABLE: bool> MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn with_value_converter<R2>(self, convert: fn(S) -> R2) -> MenuItem<T, R2, S, SELECTABLE> {
        MenuItem {
            convert,
            title_text: self.title_text,
            value: self.value,
            line: self.line,
        }
    }

    /// Make the item selectable or not
    pub fn selectable<const SELECTABLE2: bool>(self) -> MenuItem<T, R, S, SELECTABLE2> {
        MenuItem {
            convert: self.convert,
            title_text: self.title_text,
            value: self.value,
            line: self.line,
        }
    }
}

impl<T, R, S, const SELECTABLE: bool> Marker for MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
}

impl<T, R, S, const SELECTABLE: bool> MenuListItem<R> for MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
    fn value_of(&self) -> R {
        (self.convert)(self.value.clone())
    }

    fn interact(&mut self) -> R {
        self.value.next();
        self.value_of()
    }

    fn selectable(&self) -> bool {
        SELECTABLE
    }

    fn set_style(&mut self, text_style: &U8g2TextStyle<BinaryColor>) {
        let mut current = self.value.clone();

        let mut value_width_max = caculate_text_width(current.marker(), text_style);

        loop {
            current.next();
            if current == self.value {
                break;
            }

            if caculate_text_width(current.marker(), text_style) > value_width_max {
                value_width_max = caculate_text_width(current.marker(), text_style);
            }
        }

        self.line = MenuLine::new(value_width_max, text_style);
    }

    fn draw_styled<D>(
        &self,
        text_style: &U8g2TextStyle<BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.line.draw_styled(
            self.title_text.as_ref(),
            self.value.marker(),
            text_style,
            display,
        )
    }
}

impl<T, R, S, const SELECTABLE: bool> View for MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}

fn caculate_text_width(text: &str, text_style: &U8g2TextStyle<BinaryColor>) -> u32 {
    text_style
        .measure_string(text, Point::zero(), Baseline::Top)
        .bounding_box
        .size
        .width
}

#[cfg(test)]
mod test {
    #[test]
    fn interaction_selects_next_value_and_returns_converted() {
        use super::*;
        use crate::items::MenuListItem;

        let mut item = MenuItem::new("title", false).with_value_converter(|b| b as u8);

        assert_eq!(item.value_of(), 0);

        assert_eq!(item.interact(), 1);
        assert_eq!(item.value_of(), 1);

        assert_eq!(item.interact(), 0);
        assert_eq!(item.value_of(), 0);
    }
}
