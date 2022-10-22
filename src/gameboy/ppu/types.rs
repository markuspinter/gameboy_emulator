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
struct LCDControl {
    lcd_ppu_enable: bool,
    window_tile_map_area: bool,
    window_enable: bool,
    bg_and_window_tile_data_area: bool,
    bg_tile_map_area: bool,
    obj_size: bool,
    obj_enable: bool,
    //TODO: special meaning for CGB
    bg_and_window_enable: bool,
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

/*
## FF41 — STAT: LCD status

```
Bit 6 - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
Bit 1-0 - Mode Flag                          (Mode 0-3, see below) (Read Only)
          0: HBlank
          1: VBlank
          2: Searching OAM
          3: Transferring Data to LCD Controller
```
*/
#[derive(Debug)]
struct LCDStatus {
    unused_7th_bit: bool,
    lyc_interrupt_enable: bool,
    mode2_oam_interrupt_enable: bool,
    mode1_vblank_interrupt_enable: bool,
    mode0_hblank_interrupt_enable: bool,
    lyc_flag: bool,
    mode_flag: LCDModeFlag,
}

impl std::convert::From<LCDStatus> for u8 {
    fn from(stat: LCDStatus) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (stat.unused_7th_bit as u8) << 7;
        byte |= (stat.lyc_interrupt_enable as u8) << 6;
        byte |= (stat.mode2_oam_interrupt_enable as u8) << 5;
        byte |= (stat.mode1_vblank_interrupt_enable as u8) << 4;
        byte |= (stat.mode0_hblank_interrupt_enable as u8) << 3;
        byte |= (stat.lyc_flag as u8) << 2;
        byte |= (stat.mode_flag as u8);
        byte
    }
}

impl std::convert::From<u8> for LCDStatus {
    fn from(byte: u8) -> Self {
        Self {
            unused_7th_bit: ((byte >> 7) & 0b1) != 0,
            lyc_interrupt_enable: ((byte >> 6) & 0b1) != 0,
            mode2_oam_interrupt_enable: ((byte >> 5) & 0b1) != 0,
            mode1_vblank_interrupt_enable: ((byte >> 4) & 0b1) != 0,
            mode0_hblank_interrupt_enable: ((byte >> 3) & 0b1) != 0,
            lyc_flag: ((byte >> 2) & 0b1) != 0,
            mode_flag: match byte & 0b11 {
                0x0 => LCDModeFlag::HBLANK,
                0x1 => LCDModeFlag::VBLANK,
                0x2 => LCDModeFlag::SEARCHING_OAM,
                0x3 => LCDModeFlag::TRANSFERRING_DATA_TO_LCD,
                _ => panic!("LCDModeFlag convertion failed"),
            },
        }
    }
}
#[derive(Debug)]
#[repr(u8)]
enum LCDModeFlag {
    HBLANK = 0x0,
    VBLANK = 0x1,
    SEARCHING_OAM = 0x2,
    TRANSFERRING_DATA_TO_LCD = 0x3,
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

#[test]
fn stat_test() {
    let mut a: u8 = 0xA1;

    let mut stat: LCDStatus = a.into();
    println!("{:?}", stat);
    stat.lyc_interrupt_enable = true;
    stat.mode2_oam_interrupt_enable = true;
    stat.mode_flag = LCDModeFlag::TRANSFERRING_DATA_TO_LCD;
    a = u8::from(stat);
    println!("{:#04b}", a);
    assert_eq!(a, 0xE3);
}
