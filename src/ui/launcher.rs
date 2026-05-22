// App Launcher - scrollable list of games
// Tap to select, swipe up/down to scroll smoothly, swipe right to go back

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, RoundedRectangle};
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::{Alignment, Text};

use crate::apps::AppState;
use crate::board;
use crate::peripherals::touch::SwipeDirection;

const ITEM_H: i32 = 65;
const ITEM_GAP: i32 = 6;
const MARGIN_X: i32 = 20;
const START_Y: i32 = 55;
const SCREEN_W: i32 = board::LCD_WIDTH as i32;
const SCREEN_H: i32 = board::LCD_HEIGHT as i32;
const SCREEN_CX: i32 = SCREEN_W / 2;

struct MenuItem {
    name: &'static str,
    state: AppState,
    bg_color: Rgb565,
    text_color: Rgb565,
}

const MENU_ITEMS: &[MenuItem] = &[
    MenuItem { name: "Snake", state: AppState::Snake, bg_color: Rgb565::new(2, 20, 2), text_color: Rgb565::GREEN },
    MenuItem { name: "2048", state: AppState::Game2048, bg_color: Rgb565::new(15, 10, 0), text_color: Rgb565::YELLOW },
    MenuItem { name: "Tetris", state: AppState::Tetris, bg_color: Rgb565::new(0, 10, 15), text_color: Rgb565::CYAN },
    MenuItem { name: "Flappy Bird", state: AppState::Flappy, bg_color: Rgb565::new(15, 12, 0), text_color: Rgb565::WHITE },
    MenuItem { name: "Maze (Tilt)", state: AppState::Maze, bg_color: Rgb565::new(2, 4, 15), text_color: Rgb565::WHITE },
    MenuItem { name: "MP3 Player", state: AppState::Mp3Player, bg_color: Rgb565::new(0, 8, 15), text_color: Rgb565::CYAN },
    MenuItem { name: "Smart Home", state: AppState::SmartHome, bg_color: Rgb565::new(8, 4, 15), text_color: Rgb565::new(20, 10, 31) },
    MenuItem { name: "设置", state: AppState::Settings, bg_color: Rgb565::new(6, 12, 6), text_color: Rgb565::WHITE },
];

fn fill_rect<D: DrawTarget<Color = Rgb565>>(d: &mut D, x: i32, y: i32, w: u32, h: u32, color: Rgb565) {
    let _ = Rectangle::new(Point::new(x, y), Size::new(w, h))
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(d);
}

const GLYPH_SHE: [u16; 16] = [
    0x1000,
    0x0000,
    0x3E7C,
    0x1044,
    0x1044,
    0x3E7C,
    0x1010,
    0x2828,
    0x4428,
    0x8248,
    0x0148,
    0x0288,
    0x0408,
    0x0810,
    0x100E,
    0x0000,
];

const GLYPH_ZHI: [u16; 16] = [
    0xFFFE,
    0x8002,
    0xFFFE,
    0x1FF8,
    0x1110,
    0x1FF8,
    0x1110,
    0x1FF8,
    0x1110,
    0x1110,
    0x1FF8,
    0x1110,
    0x1110,
    0x1FF8,
    0xFFFE,
    0x0000,
];

fn draw_glyph<D: DrawTarget<Color = Rgb565>>(d: &mut D, x: i32, y: i32, glyph: &[u16; 16], scale: u32, color: Rgb565) {
    for (row_idx, row) in glyph.iter().enumerate() {
        for col_idx in 0..16 {
            if row & (0x8000 >> col_idx) != 0 {
                fill_rect(
                    d,
                    x + col_idx * scale as i32,
                    y + row_idx as i32 * scale as i32,
                    scale,
                    scale,
                    color,
                );
            }
        }
    }
}

fn draw_settings_cn<D: DrawTarget<Color = Rgb565>>(d: &mut D, center_x: i32, center_y: i32, color: Rgb565) {
    let scale = 2;
    let char_w = 16 * scale as i32;
    let gap = 4;
    let total_w = char_w * 2 + gap;
    let start_x = center_x - total_w / 2;
    let start_y = center_y - (16 * scale as i32) / 2;

    draw_glyph(d, start_x, start_y, &GLYPH_SHE, scale, color);
    draw_glyph(d, start_x + char_w + gap, start_y, &GLYPH_ZHI, scale, color);
}

pub struct Launcher {
    scroll_offset: i32,
    target_scroll: i32,
}

impl Launcher {
    pub fn new() -> Self {
        Self { scroll_offset: 0, target_scroll: 0 }
    }

    pub fn update(&mut self, swipe: Option<SwipeDirection>, tap: bool, tap_y: u16) -> Option<AppState> {
        let max_scroll = ((MENU_ITEMS.len() as i32) * (ITEM_H + ITEM_GAP) - (SCREEN_H - START_Y)).max(0);

        match swipe {
            Some(SwipeDirection::Up) => {
                self.target_scroll = (self.target_scroll + 120).min(max_scroll);
            }
            Some(SwipeDirection::Down) => {
                self.target_scroll = (self.target_scroll - 120).max(0);
            }
            Some(SwipeDirection::Right) => {
                return Some(AppState::Watchface);
            }
            _ => {}
        }

        let diff = self.target_scroll - self.scroll_offset;
        if diff.abs() > 1 {
            let step = ((diff * 13) / 20).clamp(-64, 64);
            self.scroll_offset += if step == 0 { diff.signum() } else { step };
        } else {
            self.scroll_offset = self.target_scroll;
        }

        if tap {
            let y = tap_y as i32 + self.scroll_offset;
            for (i, item) in MENU_ITEMS.iter().enumerate() {
                let item_y = START_Y + i as i32 * (ITEM_H + ITEM_GAP);
                if y >= item_y && y < item_y + ITEM_H {
                    return Some(item.state);
                }
            }
        }
        None
    }

    pub fn render<D: DrawTarget<Color = Rgb565>>(&self, d: &mut D) {
        let _ = Rectangle::new(Point::zero(), Size::new(SCREEN_W as u32, SCREEN_H as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(1, 2, 2)))
            .draw(d);

        let title = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        let _ = Text::with_alignment("APPS", Point::new(SCREEN_CX, 35), title, Alignment::Center).draw(d);

        for (i, item) in MENU_ITEMS.iter().enumerate() {
            let y = START_Y + i as i32 * (ITEM_H + ITEM_GAP) - self.scroll_offset;
            if y + ITEM_H < 0 || y > SCREEN_H { continue; }

            let _ = RoundedRectangle::with_equal_corners(
                Rectangle::new(Point::new(MARGIN_X, y), Size::new((SCREEN_W - 2 * MARGIN_X) as u32, ITEM_H as u32)),
                Size::new(12, 12),
            ).into_styled(PrimitiveStyle::with_fill(item.bg_color)).draw(d);

            let _ = Rectangle::new(Point::new(MARGIN_X, y + 8), Size::new(4, (ITEM_H - 16) as u32))
                .into_styled(PrimitiveStyle::with_fill(item.text_color)).draw(d);

            if item.state == AppState::Settings {
                draw_settings_cn(d, SCREEN_CX, y + ITEM_H / 2 + 1, item.text_color);
            } else {
                let text_style = MonoTextStyle::new(&FONT_10X20, item.text_color);
                let _ = Text::with_alignment(
                    item.name,
                    Point::new(SCREEN_CX, y + ITEM_H / 2 + 5),
                    text_style,
                    Alignment::Center,
                ).draw(d);
            }
        }
    }
}
