#[derive(Clone, Debug, FromPrimitive)]
enum WaveDuty {
    P12_5 = 0b00,
    P25 = 0b01,
    P50 = 0b10,
    P75 = 0b11,
}
pub struct Pulse {
    wave_duty: WaveDuty,
    wave_length: u16,
    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    noise_mpsc: mpsc::Sender<PulseParameters>,
}

impl MemoryInterface for Pulse {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR21 {
            return Some(self.get_nr21());
        } else if addr == memory::apu::NR22 {
            return Some(self.get_nr22());
        } else if addr == memory::apu::NR23 {
            return Some(self.get_nr23());
        } else if addr == memory::apu::NR24 {
            return Some(self.get_nr24());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR21 {
            self.set_nr21(value);
        } else if addr == memory::apu::NR22 {
            self.set_nr22(value);
        } else if addr == memory::apu::NR23 {
            self.set_nr23(value);
        } else if addr == memory::apu::NR24 {
            self.set_nr24(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl Pulse {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            wave_duty: WaveDuty::P12_5,
            wave_length: 0,

            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,

            shall_trigger: false,
            sound_length_enable: false,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            noise_mpsc: mpsc::channel().0,
        }
    }

    fn get_nr21(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.wave_duty as u8) << 6;
        byte |= self.length_timer & 0b11111;
        byte
    }
    fn get_nr22(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.inital_envelope_volume << 4;
        byte |= (self.envelope_increase as u8) << 3;
        byte |= self.sweep_pace & 0b111;
        byte
    }
    fn get_nr23(&self) -> u8 {
        (self.wave_length & 0xFF) as u8
    }
    fn get_nr24(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte |= ((self.wave_length >> 8) & 0b111) as u8;
        byte
    }

    fn set_nr21(&mut self, value: u8) {
        self.wave_duty = FromPrimitive::from_u8(value >> 6).expect("couldn't convert wave duty");
        self.length_timer = value & 0b11111;
    }
    fn set_nr22(&mut self, value: u8) {
        self.inital_envelope_volume = value >> 4;
        self.envelope_increase = bit!(value, 4) != 0;
        self.sweep_pace = value & 0b111;
        if !self.sink.empty() {
            if self.inital_envelope_volume == 0 {
                self.sink.pause();
                // self.sink = Sink::try_new(&self.stream_handle).unwrap(); // this is a hack, investigate why stop doesn't suffice
            } else {
                // self.sink.play();
            }
        }
    }
    fn set_nr23(&mut self, value: u8) {
        log::debug!("lower wave length before {}", self.wave_length);
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;
        let freq = 131072.0 / (2048 - self.wave_length as u32) as f32;
        log::debug!(
            "lower wave length set {}; freq {}, value {}",
            self.wave_length,
            freq,
            value
        );
        if !self.sink.empty() {
            self.noise_mpsc
                .send(PulseParameters {
                    duty_cycle: match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
    fn set_nr24(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;

        if self.sink.empty() {
            if self.shall_trigger {
                let duration =
                    Duration::from_micros((((1.0 / 256.0) * (64.0 - self.length_timer as f32)) * 1e6) as u64);
                // log::debug!("start sound for: {:?}", duration);
                let freq: f32 = 131072.0 / (2048 - self.wave_length as u32) as f32;
                let oscillator = PulseOscillator::new(
                    44100,
                    match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                );
                let res = speed::<PulseOscillator, PulseParameters>(oscillator);
                self.noise_mpsc = res.1;

                if self.sound_length_enable {
                    self.sink.append(res.0.take_duration(duration).amplify(0.1));
                } else {
                    self.sink.append(res.0.amplify(0.1));
                }
                // self.sink.append(res.0.take_duration(duration).amplify(0.1));

                log::debug!("wave length {}; freq {}", self.wave_length, freq);

                self.noise_mpsc
                    .send(PulseParameters {
                        duty_cycle: match self.wave_duty {
                            WaveDuty::P12_5 => 0.125,
                            WaveDuty::P25 => 0.25,
                            WaveDuty::P50 => 0.5,
                            WaveDuty::P75 => 0.75,
                        },
                        frequency: freq as f32,
                    })
                    .unwrap();
            }
        } else {
            let freq = 131072.0 / (2048 - self.wave_length as u32) as f32;
            log::debug!("upper wave length set {}; freq {}", self.wave_length, freq);
            self.noise_mpsc
                .send(PulseParameters {
                    duty_cycle: match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                    frequency: freq as f32,
                })
                .unwrap();
            if self.shall_trigger {
                self.sink.play();
            }
        }
    }
}

pub struct PulseSweep {
    sweep_pace_for_frequency: u8,
    sweep_decrease: bool,
    sweep_slope: u8,

    wave_duty: WaveDuty,
    wave_length: u16,
    length_timer: u8,
    inital_envelope_volume: u8,
    envelope_increase: bool,
    sweep_pace: u8,
    shall_trigger: bool,
    sound_length_enable: bool, //(1=Stop output when length in NR21 expires)
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    noise_mpsc: mpsc::Sender<PulseParameters>,
}

impl MemoryInterface for PulseSweep {
    fn read8(&self, addr: u16) -> Option<u8> {
        if addr == memory::apu::NR10 {
            return Some(self.get_nr10());
        } else if addr == memory::apu::NR11 {
            return Some(self.get_nr11());
        } else if addr == memory::apu::NR12 {
            return Some(self.get_nr12());
        } else if addr == memory::apu::NR13 {
            return Some(self.get_nr13());
        } else if addr == memory::apu::NR14 {
            return Some(self.get_nr14());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if addr == memory::apu::NR10 {
            self.set_nr10(value);
        } else if addr == memory::apu::NR11 {
            self.set_nr11(value);
        } else if addr == memory::apu::NR12 {
            self.set_nr12(value);
        } else if addr == memory::apu::NR13 {
            self.set_nr13(value);
        } else if addr == memory::apu::NR14 {
            self.set_nr14(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl PulseSweep {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            sweep_pace_for_frequency: 0,
            sweep_decrease: false,
            sweep_slope: 0,

            wave_duty: WaveDuty::P12_5,
            wave_length: 0,

            length_timer: 0,
            inital_envelope_volume: 0,
            envelope_increase: false,
            sweep_pace: 0,

            shall_trigger: false,
            sound_length_enable: false,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            noise_mpsc: mpsc::channel().0,
        }
    }

    fn get_nr10(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= ((self.sweep_pace_for_frequency & 0b111) as u8) << 4;
        byte |= (self.sweep_decrease as u8) << 3;
        byte |= self.sweep_slope & 0b111;
        byte
    }

    fn get_nr11(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.wave_duty as u8) << 6;
        byte |= self.length_timer & 0b11111;
        byte
    }
    fn get_nr12(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= self.inital_envelope_volume << 4;
        byte |= (self.envelope_increase as u8) << 3;
        byte |= self.sweep_pace & 0b111;
        byte
    }
    fn get_nr13(&self) -> u8 {
        (self.wave_length & 0xFF) as u8
    }
    fn get_nr14(&self) -> u8 {
        let mut byte: u8 = 0;
        //bit 7 trigger is read only returning 1s for this section
        byte |= 0xFF_u8 << 7;
        byte |= (self.sound_length_enable as u8) << 6;
        byte |= ((self.wave_length >> 8) & 0b111) as u8;
        byte
    }

    fn set_nr10(&mut self, value: u8) {
        self.sweep_pace_for_frequency = (value >> 4) & 0b111;
        self.sweep_decrease = bit!(value, 3) != 0;
        self.sweep_slope = value & 0b111;
    }

    fn set_nr11(&mut self, value: u8) {
        self.wave_duty = FromPrimitive::from_u8(value >> 6).expect("couldn't convert wave duty");
        self.length_timer = value & 0b11111;
    }
    fn set_nr12(&mut self, value: u8) {
        self.inital_envelope_volume = value >> 4;
        self.envelope_increase = bit!(value, 4) != 0;
        self.sweep_pace = value & 0b111;
        println!(
            "envelope volume {}; envelop increase {}",
            self.inital_envelope_volume, self.envelope_increase
        );
        if !self.sink.empty() {
            if self.inital_envelope_volume == 0 {
                self.sink.pause();
                // self.sink = Sink::try_new(&self.stream_handle).unwrap(); // this is a hack, investigate why stop doesn't suffice
            } else {
                // self.sink.play();
            }
        }
    }
    fn set_nr13(&mut self, value: u8) {
        self.wave_length &= 0x0700;
        self.wave_length |= value as u16;
        let freq = 131072.0 / (2048 - self.wave_length as u32) as f32;
        if !self.sink.empty() {
            self.noise_mpsc
                .send(PulseParameters {
                    duty_cycle: match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                    frequency: freq as f32,
                })
                .unwrap();
        }
    }
    fn set_nr14(&mut self, value: u8) {
        self.shall_trigger = bit!(value, 7) != 0;
        self.sound_length_enable = bit!(value, 6) != 0;
        self.wave_length &= 0x00FF;
        self.wave_length |= ((value & 0b111) as u16) << 8;
        println!(
            "sound length enable {}; shall trigger {}",
            self.sound_length_enable, self.shall_trigger
        );

        if self.sink.empty() {
            if self.shall_trigger {
                let duration =
                    Duration::from_micros((((1.0 / 256.0) * (64.0 - self.length_timer as f32)) * 1e6) as u64);
                let freq = 131072.0 / (2048 - self.wave_length as u32) as f32;
                // log::debug!("start sound for: {:?}", duration);
                let oscillator = PulseOscillator::new(
                    44100,
                    match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                );
                let res = speed::<PulseOscillator, PulseParameters>(oscillator);
                self.noise_mpsc = res.1;

                if self.sound_length_enable {
                    self.sink.append(res.0.take_duration(duration).amplify(0.1));
                } else {
                    self.sink.append(res.0.amplify(0.1));
                }
                // self.sink.append(res.0.take_duration(duration).amplify(0.1));

                // log::debug!("freq {}", freq);

                self.noise_mpsc
                    .send(PulseParameters {
                        duty_cycle: match self.wave_duty {
                            WaveDuty::P12_5 => 0.125,
                            WaveDuty::P25 => 0.25,
                            WaveDuty::P50 => 0.5,
                            WaveDuty::P75 => 0.75,
                        },
                        frequency: freq as f32,
                    })
                    .unwrap();
            }
        } else {
            let freq = 131072.0 / (2048 - self.wave_length as u32) as f32;
            self.noise_mpsc
                .send(PulseParameters {
                    duty_cycle: match self.wave_duty {
                        WaveDuty::P12_5 => 0.125,
                        WaveDuty::P25 => 0.25,
                        WaveDuty::P50 => 0.5,
                        WaveDuty::P75 => 0.75,
                    },
                    frequency: freq as f32,
                })
                .unwrap();
            if self.shall_trigger {
                self.sink.play();
            }
        }
    }
}

struct PulseParameters {
    duty_cycle: f32,
    frequency: f32,
}

use core::time::Duration;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};
use std::sync::mpsc;

use crate::{
    bit,
    gameboy::{apu::utils::speed, memory, MemoryInterface},
};

use super::utils::CustomSource;

pub struct PulseOscillator {
    sample_rate: u32,
    duty_cycle: f32,
    index_increment: f32,
    index: f32,
}

impl PulseOscillator {
    pub fn new(sample_rate: u32, duty_cycle: f32) -> Self {
        let mut se = Self {
            sample_rate: sample_rate,
            duty_cycle: duty_cycle,

            index_increment: 0.0,
            index: 0.0,
        };
        return se;
    }

    fn get_sample(&mut self) -> f32 {
        let sample: f32;

        // sample = self.lerp();
        sample = if (self.index) / 8.0 < self.duty_cycle { 1.0 } else { 0.0 };
        self.index += self.index_increment;
        self.index %= 8.0 as f32;

        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % 8;

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        let mut val = -1.0;
        if (self.index) / 8.0 < self.duty_cycle {
            val = 1.0;
        }
        let mut next_val = -1.0;
        if (next_index as f32) / 8.0 < self.duty_cycle {
            next_val = 1.0;
        }

        return truncated_index_weight * val + next_index_weight * next_val;
    }
}

impl CustomSource<PulseParameters> for PulseOscillator {
    fn set_parameters(&mut self, parameters: PulseParameters) {
        // self.index_increment = frequency * 64 as f32
        //                        / self.sample_rate as f32;
        log::debug!("params frequency {}", parameters.frequency);
        self.index_increment = parameters.frequency * 8.0 / self.sample_rate as f32;
    }
}

impl Source for PulseOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for PulseOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
