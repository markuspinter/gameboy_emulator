use std::{collections::VecDeque, sync::mpsc};

use rodio::{OutputStream, OutputStreamHandle, Sink};

use crate::bit;

use self::driver::{AudioDriver, AudioQueue};

use super::{memory, GameboyModule};

mod driver;
mod noise;
mod pulse;
mod utils;
mod wave;

pub struct APU {
    pulse_sweep: pulse::PulseSweep,

    pulse: pulse::Pulse,

    wave: wave::Wave,

    noise: noise::Noise,

    apu_enabled: bool,
    left_channels: [bool; Self::CHANNELS],
    right_channels: [bool; Self::CHANNELS],
    vin_left: bool,  //this is unused
    vin_right: bool, //this is unused
    left_output_volume: u8,
    right_output_volume: u8,

    pub shall_clear_audio_queue: bool,

    div: u8,

    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    audio_queue_sender: mpsc::Sender<AudioQueue>,
}

impl GameboyModule for APU {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        if self.apu_enabled {
            self.pulse_sweep.tick(gb)?;
            self.pulse.tick(gb)?;
            self.wave.tick(gb)?;
            self.noise.tick(gb)?;
        }
        Ok(0)
    }
}

impl super::MemoryInterface for APU {
    fn read8(&self, addr: u16) -> Option<u8> {
        if let Some(res) = self.pulse_sweep.read8(addr) {
            return Some(res);
        } else if let Some(res) = self.pulse.read8(addr) {
            return Some(res);
        } else if let Some(res) = self.wave.read8(addr) {
            return Some(res);
        } else if let Some(res) = self.noise.read8(addr) {
            return Some(res);
        } else if addr == memory::apu::NR50 {
            return Some(self.get_nr50());
        } else if addr == memory::apu::NR51 {
            return Some(self.get_nr51());
        } else if addr == memory::apu::NR52 {
            log::trace!("read nr52");
            return Some(self.get_nr52());
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if let Some(res) = self.pulse_sweep.write8(addr, value) {
        } else if let Some(res) = self.pulse.write8(addr, value) {
        } else if let Some(res) = self.wave.write8(addr, value) {
        } else if let Some(res) = self.noise.write8(addr, value) {
        } else if addr == memory::apu::NR50 {
            self.set_nr50(value);
        } else if addr == memory::apu::NR51 {
            self.set_nr51(value);
        } else if addr == memory::apu::NR52 {
            self.set_nr52(value);
        } else {
            return None;
        }
        return Some(());
    }
}

impl APU {
    const ENVELOPE_SWEEP_DIVIDER: u8 = 8;
    const SOUND_LENGTH_DIVIDER: u8 = 2;
    const CH1_FREQUENCY_SWEEP_DIVIDER: u8 = 4;
    const AUDIO_SAMPLING_RATE: u32 = 44100;
    const CHANNELS: usize = 4;

    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (tx, rx) = mpsc::channel();
        let mut apu = Self {
            pulse_sweep: pulse::PulseSweep::new(Self::AUDIO_SAMPLING_RATE),

            pulse: pulse::Pulse::new(Self::AUDIO_SAMPLING_RATE),

            wave: wave::Wave::new(Self::AUDIO_SAMPLING_RATE),

            noise: noise::Noise::new(Self::AUDIO_SAMPLING_RATE),

            apu_enabled: false,
            left_channels: [false; Self::CHANNELS],
            right_channels: [false; Self::CHANNELS],
            vin_left: false,  //this is unused
            vin_right: false, //this is unused
            left_output_volume: 0,
            right_output_volume: 0,
            shall_clear_audio_queue: false,

            div: 0,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            audio_queue_sender: tx,
        };
        apu.sink.append(AudioDriver::new(Self::AUDIO_SAMPLING_RATE, 2, rx));
        apu
    }

    pub fn tick_div(&mut self) {
        if self.apu_enabled {
            self.div = self.div.wrapping_add(1);

            if self.div % APU::ENVELOPE_SWEEP_DIVIDER == 0 {
                self.pulse_sweep.tick_envelope_sweep();
                self.pulse.tick_envelope_sweep();
                self.noise.tick_envelope_sweep();
            }

            if self.div % APU::SOUND_LENGTH_DIVIDER == 0 {
                self.pulse_sweep.tick_timer();
                self.pulse.tick_timer();
                self.wave.tick_timer();
                self.noise.tick_timer();
            }

            if self.div % APU::CH1_FREQUENCY_SWEEP_DIVIDER == 0 {
                // self.pulse_sweep.tick_frequency_sweep();
            }
        }
    }

    fn get_nr52(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.apu_enabled as u8) << 7;
        byte |= (self.noise.is_active() as u8) << 3;
        byte |= (self.wave.is_active() as u8) << 2;
        byte |= (self.pulse.is_active() as u8) << 1;
        byte |= (self.pulse_sweep.is_active() as u8) << 0;

        byte
    }

    fn get_nr51(&self) -> u8 {
        let mut byte: u8 = 0;
        for ch_id in 0..Self::CHANNELS {
            byte |= (self.left_channels[ch_id] as u8) << (4 + ch_id);
            byte |= (self.right_channels[ch_id] as u8) << (ch_id);
        }
        byte
    }
    fn get_nr50(&self) -> u8 {
        let mut byte: u8 = 0;
        byte |= (self.vin_left as u8) << 7;
        byte |= (self.left_output_volume & 0b111) << 4;
        byte |= (self.vin_right as u8) << 3;
        byte |= self.right_output_volume & 0b111;
        byte
    }

    fn set_nr52(&mut self, value: u8) {
        self.apu_enabled = bit!(value, 7) != 0;
        if !self.apu_enabled {
            self.div = 0;
        }
        println!("apu enabled {}", self.apu_enabled);
    }

    fn set_nr51(&mut self, value: u8) {
        for ch_id in 0..Self::CHANNELS {
            self.left_channels[ch_id] = bit!(value, 4 + ch_id) != 0;
            self.right_channels[ch_id] = bit!(value, ch_id) != 0;
        }
    }
    fn set_nr50(&mut self, value: u8) {
        self.vin_left = bit!(value, 7) != 0;
        self.left_output_volume = (value >> 4) & 0b111;
        self.vin_right = bit!(value, 3) != 0;
        self.right_output_volume = value & 0b111;
    }

    pub fn sync(&mut self) {
        let mut queue: VecDeque<f32> = VecDeque::new();
        let pulse_sweep_samples = self.pulse_sweep.get_samples();
        let pulse_samples = self.pulse.get_samples();
        let wave_samples = self.wave.get_samples();
        let noise_samples = self.noise.get_samples();
        if !(pulse_sweep_samples.len() == pulse_samples.len()
            && pulse_samples.len() == wave_samples.len()
            && wave_samples.len() == noise_samples.len())
        {
            log::warn!("samples don't have same size");
            self.shall_clear_audio_queue = true;
        }
        log::warn!(
            "pulse sweep length {}\npulse length {}\nwave length {}\nnoise length {}",
            pulse_sweep_samples.len() / 2,
            pulse_samples.len() / 2,
            wave_samples.len() / 2,
            noise_samples.len() / 2
        );
        let mut mixed_sample = 0.0;

        for i in 0..wave_samples.len() {
            // mixed_sample += pulse_sweep_samples[i];
            // mixed_sample += pulse_samples[i];
            mixed_sample += wave_samples[i];

            // mixed_sample += noise_samples[i];

            queue.push_back(mixed_sample);

            mixed_sample = 0.0;
        }

        self.audio_queue_sender
            .send(AudioQueue {
                queue,
                shall_clear_old_samples: self.shall_clear_audio_queue,
            })
            .unwrap();
        self.pulse_sweep.reset_samples();
        self.pulse.reset_samples();
        self.wave.reset_samples();
        self.noise.reset_samples();
    }
}

trait APUChannel {
    fn tick_timer(&mut self);
    fn sample(&mut self, apu: &APU);
    fn get_samples(&mut self) -> &Vec<f32>;
    fn reset_samples(&mut self);
    // fn get_current_sample(&self) -> u8; //unused right now (CGB)
    #[inline]
    fn dac(&self, apu: &APU, sample: u8, dac_enabled: bool) -> (f32, f32) {
        //-> (<left_channel>, <right_channel>)
        const SAMPLE_BIT_RESOLUTION: u8 = 15;
        const SAMPLE_VOLUME_RESOLUTION: u8 = 8;
        let mut ret = (0., 0.);
        if self.is_active() && dac_enabled {
            if apu.left_channels[2] {
                ret.0 = ((1 + apu.left_output_volume) as f32 * sample as f32)
                    / (SAMPLE_BIT_RESOLUTION * SAMPLE_VOLUME_RESOLUTION) as f32
            }
            if apu.right_channels[2] {
                ret.1 = ((1 + apu.right_output_volume) as f32 * sample as f32)
                    / (SAMPLE_BIT_RESOLUTION * SAMPLE_VOLUME_RESOLUTION) as f32
            }
        }
        ret
    }
    fn is_active(&self) -> bool;
}

trait APUEnvelope {
    fn tick_envelope_sweep(&mut self);
}
