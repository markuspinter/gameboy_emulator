use crate::bit;

/*
### FF47 — BGP (Non-CGB Mode only): BG palette data

This register assigns gray shades to the color indexes of the BG and
Window tiles.

```
Bit 7-6 - Color for index 3
Bit 5-4 - Color for index 2
Bit 3-2 - Color for index 1
Bit 1-0 - Color for index 0
```

Value | Color
------|-------
  0   | White
  1   | Light gray
  2   | Dark gray
  3   | Black
*/
#[derive(Clone, Debug)]
pub struct PaletteData {
    pub color_map: [u8; 4],
}

impl std::convert::From<PaletteData> for u8 {
    fn from(pd: PaletteData) -> u8 {
        let mut byte: u8 = 0x00;
        byte |= (pd.color_map[3] & 0b11) << 6;
        byte |= (pd.color_map[2] & 0b11) << 4;
        byte |= (pd.color_map[1] & 0b11) << 2;
        byte |= pd.color_map[0] & 0b11;
        byte
    }
}

impl std::convert::From<u8> for PaletteData {
    fn from(byte: u8) -> Self {
        let mut color_map = [0; 4];
        color_map[3] = bit!(byte, 7) << 1 | bit!(byte, 6);
        color_map[2] = bit!(byte, 5) << 1 | bit!(byte, 4);
        color_map[1] = bit!(byte, 3) << 1 | bit!(byte, 2);
        color_map[0] = bit!(byte, 1) << 1 | bit!(byte, 0);
        Self { color_map }
    }
}

// pub enum MonochromeColor {
//     Off = 0x00CADC9F,
//     White = 0x009BBC0F,
//     LightGray = 0x008BAC0F,
//     DarkGray = 0x00306230,
//     Black = 0x000F380F,
// }

pub enum MonochromeColor {
    Off = 0x00FFFFAA,
    White = 0x00FFFFFF,
    LightGray = 0x00AAAAAA,
    DarkGray = 0x00666666,
    Black = 0x00000000,
}

pub enum _MonochromeColorID {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

#[test]
fn lcdc_test() {
    let mut a: u8 = 0xA1;

    let mut pd: PaletteData = a.into();
    println!("{:?}", pd);
    pd.color_map[0] = 3;
    pd.color_map[2] = 1;
    println!("{:?}", pd);
    a = u8::from(pd);
    println!("{:#04X}", a);

    assert_eq!(a, 0x93);
}
