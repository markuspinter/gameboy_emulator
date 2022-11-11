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
#[derive(Clone, Debug)]
pub struct LCDStatus {
    pub unused_7th_bit: bool,
    pub lyc_interrupt_enable: bool,
    pub mode2_oam_interrupt_enable: bool,
    pub mode1_vblank_interrupt_enable: bool,
    pub mode0_hblank_interrupt_enable: bool,
    pub lyc_flag: bool,
    pub mode_flag: LCDModeFlag,
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
        byte |= stat.mode_flag as u8;
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
                0x0 => LCDModeFlag::HBlank,
                0x1 => LCDModeFlag::VBlank,
                0x2 => LCDModeFlag::SearchingOAM,
                0x3 => LCDModeFlag::TransferringDataToLCD,
                _ => panic!("LCDModeFlag convertion failed"),
            },
        }
    }
}
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum LCDModeFlag {
    HBlank = 0x0,
    VBlank = 0x1,
    SearchingOAM = 0x2,
    TransferringDataToLCD = 0x3,
}

#[test]
fn stat_test() {
    let mut a: u8 = 0xA1;

    let mut stat: LCDStatus = a.into();
    println!("{:?}", stat);
    stat.lyc_interrupt_enable = true;
    stat.mode2_oam_interrupt_enable = true;
    stat.mode_flag = LCDModeFlag::TransferringDataToLCD;
    a = u8::from(stat);
    println!("{:#04b}", a);
    assert_eq!(a, 0xE3);
}
