/*
## FF40 — LCDC: LCD control

**LCDC** is the main **LCD C**ontrol register. Its bits toggle what
elements are displayed on the screen, and how.

Bit | Name                           | Usage notes
----|--------------------------------|-------------------------
 7  | LCD and PPU enable             | 0=Off, 1=On
 6  | Window tile map area           | 0=9800-9BFF, 1=9C00-9FFF
 5  | Window enable                  | 0=Off, 1=On
 4  | BG and Window tile data area   | 0=8800-97FF, 1=8000-8FFF
 3  | BG tile map area               | 0=9800-9BFF, 1=9C00-9FFF
 2  | OBJ size                       | 0=8x8, 1=8x16
 1  | OBJ enable                     | 0=Off, 1=On
 0  | BG and Window enable/priority  | 0=Off, 1=On
  */
#[derive(Debug)]
pub struct LCDControl {
    pub lcd_ppu_enable: bool,
    pub window_tile_map_area: bool,
    pub window_enable: bool,
    pub bg_and_window_tile_data_area: bool,
    pub bg_tile_map_area: bool,
    pub obj_size: bool,
    pub obj_enable: bool,
    //TODO: special meaning for CGB
    pub bg_and_window_enable: bool,
}

impl std::convert::From<LCDControl> for u8 {
    fn from(lcdc: LCDControl) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (lcdc.lcd_ppu_enable as u8) << 7;
        byte |= (lcdc.window_tile_map_area as u8) << 6;
        byte |= (lcdc.window_enable as u8) << 5;
        byte |= (lcdc.bg_and_window_tile_data_area as u8) << 4;
        byte |= (lcdc.bg_tile_map_area as u8) << 3;
        byte |= (lcdc.obj_size as u8) << 2;
        byte |= (lcdc.obj_enable as u8) << 1;
        byte |= (lcdc.bg_and_window_enable as u8);
        byte
    }
}

impl std::convert::From<u8> for LCDControl {
    fn from(byte: u8) -> Self {
        Self {
            lcd_ppu_enable: ((byte >> 7) & 0b1) != 0,
            window_tile_map_area: ((byte >> 6) & 0b1) != 0,
            window_enable: ((byte >> 5) & 0b1) != 0,
            bg_and_window_tile_data_area: ((byte >> 4) & 0b1) != 0,
            bg_tile_map_area: ((byte >> 3) & 0b1) != 0,
            obj_size: ((byte >> 2) & 0b1) != 0,
            obj_enable: ((byte >> 1) & 0b1) != 0,
            bg_and_window_enable: (byte & 0b1) != 0,
        }
    }
}

#[test]
fn lcdc_test() {
    let mut a: u8 = 0xA1;

    let mut lcdc: LCDControl = a.into();
    lcdc.obj_enable = true;
    lcdc.lcd_ppu_enable = true;
    lcdc.window_enable = false;
    a = u8::from(lcdc);
    println!("{:#04X}", a);
    assert_eq!(a, 0x83);
}
