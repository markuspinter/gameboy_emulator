/*
## Byte 3 — Attributes/Flags

```
 Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
 Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
 Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
 Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
 Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
 Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
```
*/

use crate::{bit, gameboy::memory};

//TODO: support cgb mode
#[derive(Debug)]
struct SpriteAttributes {
    bg_window_override: bool,
    y_flip: bool,
    x_flip: bool,
    palette_number: u8,
    tile_vram_bank_cgb: u8, //CGB Only
    palette_number_cgb: u8, //CGB Only
}

impl std::convert::From<SpriteAttributes> for u8 {
    fn from(attr: SpriteAttributes) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (attr.bg_window_override as u8) << 7;
        byte |= (attr.y_flip as u8) << 6;
        byte |= (attr.x_flip as u8) << 5;
        byte |= (attr.palette_number & 0b1) << 4;
        byte |= (attr.tile_vram_bank_cgb & 0b1) << 3;
        byte |= attr.palette_number_cgb & 0b111;
        byte
    }
}

impl std::convert::From<u8> for SpriteAttributes {
    fn from(byte: u8) -> Self {
        Self {
            bg_window_override: bit!(byte, 7) != 0,
            y_flip: bit!(byte, 6) != 0,
            x_flip: bit!(byte, 5) != 0,
            palette_number: bit!(byte, 4),
            tile_vram_bank_cgb: bit!(byte, 3),
            palette_number_cgb: byte & 0b111,
        }
    }
}

#[derive(Debug)]
struct OAMTableEntry {
    x_pos: i16,
    y_pos: i16,
    tile_index: u8,
    attributes: SpriteAttributes,
}

impl OAMTableEntry {
    const Y_POS_OFFSET: u16 = 0;
    const X_POS_OFFSET: u16 = 1;
    const TILE_INDEX_OFFSET: u16 = 2;
    const ATTRIBUTES_OFFSET: u16 = 3;
    const SIZE: u16 = 4;

    pub fn new(oam: &[u8; memory::ppu::OAM.size], start_address: u16) -> Self {
        if start_address + Self::SIZE > oam.len() as u16 {
            panic!(
                "invalid oam entry, {} required, but {} bytes left",
                Self::SIZE,
                (oam.len() as u16 - start_address)
            );
        }
        Self {
            x_pos: (oam[(start_address + Self::X_POS_OFFSET) as usize]) as i16 - 8,
            y_pos: (oam[(start_address + Self::Y_POS_OFFSET) as usize]) as i16 - 16,
            tile_index: oam[(start_address + Self::TILE_INDEX_OFFSET) as usize],
            attributes: oam[(start_address + Self::ATTRIBUTES_OFFSET) as usize].into(),
        }
    }
}

#[test]
fn attr_test() {
    let mut a: u8 = 0xA1;

    let mut attr: SpriteAttributes = a.into();
    println!("{:?}", attr);
    attr.x_flip = true;
    attr.bg_window_override = false;
    attr.palette_number_cgb = 4;
    println!("{:?}", attr);
    a = u8::from(attr);
    println!("{:#04X}", a);
    assert_eq!(a, 0x24);
}
