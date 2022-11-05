pub enum LFSRWidth {
    LFSR7Bits,
    LFSR15Bits,
}
pub struct Noise {
    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    clock_shift: u8,
    lfsr_width: LFSRWidth,
    clock_divider: u8,
    shall_trigger: bool,
    sound_length_enable: u8, //(1=Stop output when length in NR41 expires)
}
