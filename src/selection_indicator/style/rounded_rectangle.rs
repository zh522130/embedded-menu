use embedded_graphics::{
    geometry::Size,
    prelude::DrawTarget,
    primitives::{
        CornerRadii, Primitive, PrimitiveStyle, Rectangle as RectangleShape,
        RoundedRectangle as RoundedRectangleShape,
    },
    Drawable,
};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
    theme::Theme,
};

/// A rounded rectangle selection indicator
#[derive(Clone, Copy)]
pub struct RoundedRectangle {
    /// Corner radius of the rectangle
    corner_radius: u32,
    /// Left padding
    padding_left: i32,
    /// Top padding
    padding_top: i32,
    /// Right padding
    padding_right: i32,
    /// Bottom padding
    padding_bottom: i32,
}

impl Default for RoundedRectangle {
    fn default() -> Self {
        Self::new(2)
    }
}

impl RoundedRectangle {
    /// Creates a new rounded rectangle indicator with the specified corner radius
    pub const fn new(corner_radius: u32) -> Self {
        Self {
            corner_radius,
            padding_left: 2,
            padding_top: 0,
            padding_right: 1,
            padding_bottom: 0,
        }
    }

    /// Sets the padding for all sides of the indicator
    pub const fn with_padding(mut self, left: i32, top: i32, right: i32, bottom: i32) -> Self {
        self.padding_left = left;
        self.padding_top = top;
        self.padding_right = right;
        self.padding_bottom = bottom;
        self
    }
}

impl IndicatorStyle for RoundedRectangle {
    type Shape = RoundedRectangleShape;
    type State = ();

    fn padding(&self, _state: &Self::State, _height: i32) -> Insets {
        Insets {
            left: self.padding_left,
            top: self.padding_top,
            right: self.padding_right,
            bottom: self.padding_bottom,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: RectangleShape, fill_width: u32) -> Self::Shape {
        // Use fill_width for progress bar effect if available
        let width = if fill_width > 0 {
            fill_width
        } else {
            bounds.size.width
        };

        let rect = RectangleShape::new(bounds.top_left, Size::new(width, bounds.size.height));

        // Create uniform corner radii
        let corners = CornerRadii::new(Size::new(self.corner_radius, self.corner_radius));

        RoundedRectangleShape::new(rect, corners)
    }

    fn draw<T, D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        theme: &T,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        T: Theme,
        D: DrawTarget<Color = T::Color>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        shape
            .into_styled(PrimitiveStyle::with_fill(theme.selection_color()))
            .draw(display)?;

        Ok(shape)
    }
}
