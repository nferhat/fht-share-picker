use iced::widget::canvas::{self, Stroke};
use iced::{mouse, Point, Rectangle, Renderer, Vector};

use crate::theme::Theme;

#[derive(Default)]
pub struct OutputGrid {
    cache: canvas::Cache,
    outputs: Vec<OutputData>,
}

impl OutputGrid {
    pub fn new(outputs: Vec<OutputData>) -> Self {
        Self {
            cache: canvas::Cache::default(),
            outputs,
        }
    }

    fn calculate_scale_factor(&self, bounds: Rectangle) -> (Rectangle, f32) {
        let output_total_bounds = self
            .outputs
            .iter()
            .fold(Rectangle::default(), |acc, o| acc.union(&o.rec));
        // Try on the width and the height.
        // Use the smallest one
        let scale_factor_x = bounds.width / output_total_bounds.width;
        let scale_factor_y = bounds.height / output_total_bounds.height;
        let scale_factor = scale_factor_x.min(scale_factor_y);
        (output_total_bounds * scale_factor, scale_factor)
    }
}

impl canvas::Program<String, Theme> for OutputGrid {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<String>) {
        self.cache.clear(); // PERF: This is horrible for obvious reasons.

        // If the cursor is inside our bounds, then proceed to update
        let Some(position) = cursor.position_in(bounds) else {
            return (canvas::event::Status::Ignored, None);
        };

        let (scaled_output_bounds, scale_factor) = self.calculate_scale_factor(bounds);
        // Center the outputs
        let start_x = (bounds.width - scaled_output_bounds.width) / 2.0;
        let start_y = (bounds.height - scaled_output_bounds.height) / 2.0;

        for output in &self.outputs {
            let mut rec = output.rec * scale_factor;
            rec.x += start_x;
            rec.y += start_y;

            let just_clicked = matches!(
                event,
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            );
            if rec.contains(position) && just_clicked {
                return (canvas::event::Status::Captured, Some(output.name.clone()));
            }
        }

        // Nothing interesting here
        (canvas::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        // We need to map all our outputs to a certain canvas.
        //
        // To achieve this, we calculate the *real* pixel size of all the outputs grouped together
        let (scaled_output_bounds, scale_factor) = self.calculate_scale_factor(bounds);

        // Center the outputs
        let start_x = (bounds.width - scaled_output_bounds.width) / 2.0;
        let start_y = (bounds.height - scaled_output_bounds.height) / 2.0;

        let outputs = self.cache.draw(renderer, bounds.size(), |frame| {
            for output in &self.outputs {
                let mut rec = output.rec * scale_factor;
                rec.x += start_x;
                rec.y += start_y;
                // TODO: This kind of padding is meh, since it also adds to the position which is
                // not idea.
                rec.x += OUTPUT_SPACING;
                rec.y += OUTPUT_SPACING;
                rec.width -= OUTPUT_SPACING * 2.0;
                rec.height -= OUTPUT_SPACING * 2.0;
                // also make sure its centered (i hate this)

                let cursor_inside =
                    matches!(cursor.position_in(bounds), Some(p) if rec.contains(p));

                frame.fill_rectangle(rec.position(), rec.size(), theme.background.tertiary);

                // Make an outline to separate outputs.
                //
                // Assume our output have these four points, A B C D, like so
                // a --- b
                // |     |
                // d --- c
                let position = rec.position();
                let size = rec.size();
                let a = position;
                let b = Point {
                    x: position.x + size.width,
                    y: position.y,
                };
                let c = Point {
                    x: position.x + size.width,
                    y: position.y + size.height,
                };
                let d = Point {
                    x: position.x,
                    y: position.y + size.height,
                };
                let path = canvas::Path::new(|builder| {
                    builder.move_to(a);
                    builder.line_to(b);

                    builder.move_to(b);
                    builder.line_to(c);

                    builder.move_to(c);
                    builder.line_to(d);

                    builder.move_to(d);
                    builder.line_to(a);
                });
                frame.stroke(
                    &path,
                    Stroke {
                        width: 2.0,
                        style: canvas::Style::Solid(if cursor_inside {
                            theme.accent
                        } else {
                            theme.text.tertiary
                        }),
                        line_cap: canvas::LineCap::Square,
                        line_join: canvas::LineJoin::Bevel,
                        ..Default::default()
                    },
                );

                // The text should be centered.

                frame.fill_text(canvas::Text {
                    content: output.name.to_string(),
                    position: rec.center() - Vector::new(0.0, crate::TEXT_SIZE / 2.0 + 1.0),
                    size: crate::TEXT_SIZE.into(),
                    color: theme.accent,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    font: crate::font::MONO_BOLD,
                    ..Default::default()
                });

                let info = canvas::Text {
                    content: format!(
                        "{}x{} @ {}hz",
                        output.rec.width, output.rec.height, output.refresh
                    ),
                    position: rec.center() + Vector::new(0.0, crate::TEXT_SIZE / 2.0 + 1.0),
                    size: crate::TEXT_SIZE.into(),
                    color: theme.text.tertiary,
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    font: crate::font::MONO,
                    ..Default::default()
                };
                frame.fill_text(info);
            }
        });

        vec![outputs]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::Grab
    }
}

#[derive(Debug, Clone)]
pub struct OutputData {
    pub name: String,
    pub rec: Rectangle<f32>,
    pub refresh: f32,
}

const OUTPUT_SPACING: f32 = 4.0;
