// PCM Recorder - records raw 8 kHz 16-bit samples to SD /mp3/ folder

use embedded_graphics::geometry::Point as EgPoint;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle, RoundedRectangle};
use embedded_graphics::text::{Alignment, Text};

use crate::apps::{App, AppInput, AppResult};
use crate::board;

const W: i32 = board::LCD_WIDTH as i32;
const H: i32 = board::LCD_HEIGHT as i32;

#[derive(Clone, Copy, PartialEq)]
pub enum RecorderState {
    Stopped,
    Recording,
    Error,
}

pub struct RecorderApp {
    state: RecorderState,
    toggle_requested: bool,
    bytes_written: u32,
    file_name: [u8; 16],
    file_name_len: usize,
    status: [u8; 32],
    status_len: usize,
}

impl RecorderApp {
    pub fn new() -> Self {
        let mut app = Self {
            state: RecorderState::Stopped,
            toggle_requested: false,
            bytes_written: 0,
            file_name: [0; 16],
            file_name_len: 0,
            status: [0; 32],
            status_len: 0,
        };
        app.set_file_name("--.PCM");
        app.set_status("Tap to record");
        app
    }

    pub fn state(&self) -> RecorderState { self.state }

    pub fn take_toggle_request(&mut self) -> bool {
        let requested = self.toggle_requested;
        self.toggle_requested = false;
        requested
    }

    pub fn set_recording(&mut self, file_name: &str) {
        self.state = RecorderState::Recording;
        self.bytes_written = 0;
        self.set_file_name(file_name);
        self.set_status("Recording");
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        self.bytes_written = self.bytes_written.saturating_add(bytes as u32);
    }

    pub fn set_stopped(&mut self) {
        self.state = RecorderState::Stopped;
        self.set_status("Saved");
    }

    pub fn set_error(&mut self, status: &str) {
        self.state = RecorderState::Error;
        self.set_status(status);
    }

    fn set_file_name(&mut self, file_name: &str) {
        let bytes = file_name.as_bytes();
        let len = bytes.len().min(self.file_name.len());
        self.file_name[..len].copy_from_slice(&bytes[..len]);
        self.file_name_len = len;
    }

    fn set_status(&mut self, status: &str) {
        let bytes = status.as_bytes();
        let len = bytes.len().min(self.status.len());
        self.status[..len].copy_from_slice(&bytes[..len]);
        self.status_len = len;
    }

    fn file_name_str(&self) -> &str {
        core::str::from_utf8(&self.file_name[..self.file_name_len]).unwrap_or("??.PCM")
    }

    fn status_str(&self) -> &str {
        core::str::from_utf8(&self.status[..self.status_len]).unwrap_or("Error")
    }
}

impl App for RecorderApp {
    fn name(&self) -> &str { "Recorder" }

    fn setup(&mut self) {
        self.toggle_requested = false;
        if self.state != RecorderState::Recording {
            self.state = RecorderState::Stopped;
            self.bytes_written = 0;
            self.set_status("Tap to record");
        }
    }

    fn update(&mut self, input: &AppInput) -> AppResult {
        if input.tap {
            self.toggle_requested = true;
        }
        AppResult::Continue
    }

    fn render<D: DrawTarget<Color = Rgb565>>(&self, d: &mut D) {
        let _ = Rectangle::new(EgPoint::zero(), Size::new(W as u32, H as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(d);

        let white = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        let cyan = MonoTextStyle::new(&FONT_10X20, Rgb565::CYAN);
        let dim = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_GRAY);

        let _ = Text::with_alignment("RECORDER", EgPoint::new(W / 2, 40), cyan, Alignment::Center).draw(d);

        let (state_text, state_color) = match self.state {
            RecorderState::Stopped => ("STOPPED", Rgb565::CSS_GRAY),
            RecorderState::Recording => ("RECORDING", Rgb565::RED),
            RecorderState::Error => ("ERROR", Rgb565::YELLOW),
        };

        let _ = Circle::new(EgPoint::new(W / 2 - 55, 95), 110)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(12, 2, 2)))
            .draw(d);
        let _ = Circle::new(EgPoint::new(W / 2 - 25, 125), 50)
            .into_styled(PrimitiveStyle::with_fill(state_color))
            .draw(d);

        let _ = Text::with_alignment(
            state_text,
            EgPoint::new(W / 2, 245),
            MonoTextStyle::new(&FONT_10X20, state_color),
            Alignment::Center,
        ).draw(d);

        let _ = Text::with_alignment(self.status_str(), EgPoint::new(W / 2, 280), white, Alignment::Center).draw(d);

        let mut path = [0u8; 24];
        let path_len = fmt_path(&mut path, self.file_name_str());
        let _ = Text::with_alignment(path_len, EgPoint::new(W / 2, 315), dim, Alignment::Center).draw(d);

        let mut bytes = [0u8; 24];
        let bytes_text = fmt_bytes(&mut bytes, self.bytes_written);
        let _ = Text::with_alignment(bytes_text, EgPoint::new(W / 2, 345), dim, Alignment::Center).draw(d);

        let _ = RoundedRectangle::with_equal_corners(
            Rectangle::new(EgPoint::new((W - 260) / 2, 375), Size::new(260, 48)),
            Size::new(12, 12),
        ).into_styled(PrimitiveStyle::with_stroke(Rgb565::CYAN, 2)).draw(d);

        let hint = if self.state == RecorderState::Recording { "TAP: Stop" } else { "TAP: Start" };
        let _ = Text::with_alignment(hint, EgPoint::new(W / 2, 407), cyan, Alignment::Center).draw(d);
        let _ = Text::with_alignment("BOOT: Back", EgPoint::new(W / 2, H - 20), dim, Alignment::Center).draw(d);
    }
}

fn fmt_path<'a>(buf: &'a mut [u8; 24], file_name: &str) -> &'a str {
    let mut p = 0;
    for &b in b"/mp3/" { buf[p] = b; p += 1; }
    for &b in file_name.as_bytes() {
        if p >= buf.len() { break; }
        buf[p] = b.to_ascii_lowercase();
        p += 1;
    }
    core::str::from_utf8(&buf[..p]).unwrap_or("/mp3/??.pcm")
}

fn fmt_bytes<'a>(buf: &'a mut [u8; 24], mut value: u32) -> &'a str {
    let mut p = 0;
    for &b in b"Bytes " { buf[p] = b; p += 1; }
    if value == 0 {
        buf[p] = b'0';
        p += 1;
    } else {
        let mut digits = [0u8; 10];
        let mut n = 0;
        while value > 0 && n < digits.len() {
            digits[n] = b'0' + (value % 10) as u8;
            value /= 10;
            n += 1;
        }
        while n > 0 {
            n -= 1;
            buf[p] = digits[n];
            p += 1;
        }
    }
    core::str::from_utf8(&buf[..p]).unwrap_or("Bytes ?")
}
