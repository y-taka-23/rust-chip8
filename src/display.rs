use iced::widget::canvas::{Canvas, Cursor, Frame, Geometry, Program};
use iced::{Color, Element, Length, Point, Rectangle, Size};

pub const WIDTH: usize = PIXEL_SIZE * DISPLAY_WIDTH + DISPLAY_FRAME * 2;
pub const HEIGHT: usize = PIXEL_SIZE * DISPLAY_HEIGHT + DISPLAY_FRAME * 2;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_FRAME: usize = 5;
const PIXEL_SIZE: usize = 10;
const PIXEL_GAP: usize = 1;

pub struct Display {
    at: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pixel_color: iced::Color,
    background_color: iced::Color,
}

impl Display {
    pub fn new(pixel_color: Color) -> Self {
        let darken = 0.1;
        let background_color = Color::new(
            darken * pixel_color.r,
            darken * pixel_color.g,
            darken * pixel_color.b,
            1.0,
        );
        Display {
            at: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            pixel_color,
            background_color,
        }
    }

    pub fn clear(&mut self) {
        self.at = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (offset_y, line) in sprite.iter().enumerate() {
            let wrapped_y = (y as usize + offset_y) % DISPLAY_HEIGHT;
            for offset_x in 0..8 {
                let wrapped_x = (x as usize + offset_x) % DISPLAY_WIDTH;
                let old = self.at[wrapped_y][wrapped_x];
                let new = (line >> (7 - offset_x)) % 2 == 1;
                self.at[wrapped_y][wrapped_x] = old ^ new;
                if old && new {
                    collision = true;
                }
            }
        }

        collision
    }

    pub fn view(&mut self) -> Element<()> {
        Canvas::new(self)
            .width(Length::Units(
                (PIXEL_SIZE * DISPLAY_WIDTH + DISPLAY_FRAME * 2) as u16,
            ))
            .height(Length::Units(
                (PIXEL_SIZE * DISPLAY_HEIGHT + DISPLAY_FRAME * 2) as u16,
            ))
            .into()
    }
}

impl Program<()> for Display {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        frame.fill_rectangle(Point::ORIGIN, bounds.size(), self.background_color);
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                if self.at[y][x] {
                    frame.fill_rectangle(
                        Point::new(
                            (x * PIXEL_SIZE + DISPLAY_FRAME) as f32,
                            (y * PIXEL_SIZE + DISPLAY_FRAME) as f32,
                        ),
                        Size::new(
                            (PIXEL_SIZE - PIXEL_GAP) as f32,
                            (PIXEL_SIZE - PIXEL_GAP) as f32,
                        ),
                        self.pixel_color,
                    );
                }
            }
        }
        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_once() {
        let mut display = Display::new(Color::WHITE);

        display.clear();
        assert_eq!(display.at, Display::new(Color::WHITE).at);
    }

    #[test]
    fn clear_twice() {
        let mut display = Display::new(Color::WHITE);

        display.clear();
        display.clear();
        assert_eq!(display.at, Display::new(Color::WHITE).at);
    }

    #[test]
    fn clear_after_draw() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xFF; 8];

        display.draw_sprite(0, 0, sprite);
        display.clear();
        assert_eq!(display.at, Display::new(Color::WHITE).at);
    }

    #[test]
    fn draw_single_sprite_without_wrap() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xC0; 2];

        let collision = display.draw_sprite(0, 0, sprite);
        assert!(display.at[0][0]);
        assert!(display.at[0][1]);
        assert!(display.at[1][0]);
        assert!(display.at[1][1]);
        assert!(!collision);
    }

    #[test]
    fn draw_single_sprite_with_wrap() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xC0; 2];

        let (max_x, max_y) = (DISPLAY_WIDTH as u8 - 1, DISPLAY_HEIGHT as u8 - 1);
        let collision = display.draw_sprite(max_x, max_y, sprite);
        assert!(display.at[0][0]);
        assert!(display.at[0][max_x as usize]);
        assert!(display.at[max_y as usize][0]);
        assert!(display.at[max_y as usize][max_x as usize]);
        assert!(!collision);
    }

    #[test]
    fn draw_single_sprite_twice() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xC0; 2];

        display.draw_sprite(0, 0, sprite);
        let collision = display.draw_sprite(0, 0, sprite);
        assert_eq!(display.at, Display::new(Color::WHITE).at);
        assert!(collision);
    }

    #[test]
    fn draw_sprites_without_collision() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xF0, 0xF0, 0xF0, 0xF0, 0x00, 0x00, 0x00, 0x00];

        display.draw_sprite(0, 0, sprite);
        let collision = display.draw_sprite(4, 4, sprite);
        assert!(!collision);
    }

    #[test]
    fn draw_sprites_with_collision() {
        let mut display = Display::new(Color::WHITE);
        let sprite: &[u8] = &[0xF0, 0xF0, 0xF0, 0xF0, 0x00, 0x00, 0x00, 0x00];

        display.draw_sprite(0, 0, sprite);
        let collision = display.draw_sprite(3, 3, sprite);
        assert!(collision);
    }
}
