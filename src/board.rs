// Board definitions for Waveshare ESP32-S3-Touch-AMOLED-1.8
// References: ESP32-S3-Touch-AMOLED-1.8 BSP headers and bundled vendor examples

// === QSPI Display (SH8601/368x448) ===
pub const LCD_SDIO0: u8 = 4;
pub const LCD_SDIO1: u8 = 5;
pub const LCD_SDIO2: u8 = 6;
pub const LCD_SDIO3: u8 = 7;
pub const LCD_SCLK: u8 = 11;
pub const LCD_CS: u8 = 12;
pub const LCD_WIDTH: u16 = 368;
pub const LCD_HEIGHT: u16 = 448;
pub const LCD_COL_OFFSET: u16 = 0;
pub const LCD_ROW_OFFSET: u16 = 0;
pub const LCD_TE: u8 = 13;

// TCA9554 output index used for panel reset in vendor examples.
pub const LCD_RESET: u8 = 0;

// === I2C Bus ===
pub const I2C_SDA: u8 = 15;
pub const I2C_SCL: u8 = 14;
pub const I2C_FREQ_HZ: u32 = 400_000;

// === Touch (FT3168 / FT5x06-compatible) ===
pub const TP_INT: u8 = 21;
pub const TP_I2C_ADDR: u8 = 0x38;

// TCA9554 output index used for touch reset in vendor examples.
pub const TP_RESET: u8 = 2;

// === Power / sensors on shared I2C bus ===
pub const PMIC_I2C_ADDR: u8 = 0x34;
pub const IMU_I2C_ADDR: u8 = 0x6B;
pub const RTC_I2C_ADDR: u8 = 0x51;

// === SD Card ===
pub const SD_CLK: u8 = 2;
pub const SD_CMD: u8 = 1;
pub const SD_DATA: u8 = 3;

// TCA9554 output index used as SD chip select in the current Rust wiring.
pub const SD_CS: u8 = 7;

// === Audio I2S ===
pub const I2S_MCLK: u8 = 16;
pub const I2S_SCLK: u8 = 9;
pub const I2S_LRCK: u8 = 45;
pub const I2S_DOUT: u8 = 8;
pub const I2S_DIN: u8 = 10;
pub const PA_CTRL: u8 = 46;

// === IO Expander ===
pub const EXIO_I2C_ADDR: u8 = 0x20;
pub const DSI_PWR_EN: u8 = 1;
