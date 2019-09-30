use ::volatile::Volatile;
use ::core::fmt;
use ::spin::Mutex;

lazy_static! {
    pub static ref STD_OUT: Mutex<Writer> = Mutex::new(Writer::new(unsafe { &mut *(0xb8000 as *mut Buffer) }, ColourCode::new(Colour::Yellow, Colour::Black)));
}

// Print macros are taken straight from stdlib.

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    STD_OUT.lock().write_fmt(args).unwrap();
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Colour {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Brown,
    LightGrey,
    DarkGrey,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl Colour {
    pub fn from_u8(n: u8) -> Option<Colour> {
        use self::Colour::*;
        match n {
            0  => Some(Black),
            1  => Some(DarkBlue),
            2  => Some(DarkGreen),
            3  => Some(DarkCyan),
            4  => Some(DarkRed),
            5  => Some(Purple),
            6  => Some(Brown),
            7  => Some(LightGrey),
            8  => Some(DarkGrey),
            9  => Some(LightBlue),
            10 => Some(LightGreen),
            11 => Some(LightCyan),
            12 => Some(LightRed),
            13 => Some(Pink),
            14 => Some(Yellow),
            15 => Some(White),
            _  => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColourCode {
    pub foreground: Colour,
    pub background: Colour,
}

impl ColourCode {
    pub fn new(foreground: Colour, background: Colour) -> Self {
        ColourCode {
            foreground,
            background,
        }
    }

    pub fn to_vga(&self) -> u8 {
        (self.background as u8) << 4 | (self.foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    colour: u8,
}

pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    row: usize,
    pub colour: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(buffer: &'static mut Buffer, colour: ColourCode) -> Self {
        let mut writer = Writer {
            column_pos: 0,
            row: 0,
            colour,
            buffer,
        };
        writer.clear_screen();
        writer
    }

    pub fn write(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
            return
        }
        if self.column_pos >= BUFFER_WIDTH {
            self.new_line();
        }
        self.buffer.chars[self.row][self.column_pos].write(ScreenChar {
            ascii_character: byte,
            colour: self.colour.to_vga(),
        });
        self.column_pos += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                0x20..=0x7e | b'\n' => self.write(b),
                _ => self.write(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        if self.row < BUFFER_HEIGHT - 1 {
            self.row += 1;
            self.column_pos = 0;
            return
        }
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour: self.colour.to_vga(),
        };
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(blank);
        }
    }

    pub fn clear_screen(&mut self) {
        for i in 0..BUFFER_HEIGHT {
            self.clear_row(i);
        }
        self.column_pos = 0;
        self.row = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
