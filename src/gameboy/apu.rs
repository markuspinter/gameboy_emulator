use std::{collections::VecDeque, sync::mpsc};

use rodio::{OutputStream, OutputStreamHandle, Sink};

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

    nr50: u8,
    nr51: u8,
    nr52: u8,

    div: u8,

    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    audio_queue_sender: mpsc::Sender<AudioQueue>,
}

impl GameboyModule for APU {
    unsafe fn tick(&mut self, gb_ptr: *mut crate::gameboy::Gameboy) -> Result<u32, std::fmt::Error> {
        let gb = &mut *gb_ptr;
        if gb.vblank {
            let mut queue: VecDeque<f32> = VecDeque::new();
            let pulse_sweep_samples = self.pulse_sweep.get_samples();
            let pulse_samples = self.pulse.get_samples();
            for (i, sample) in self.wave.get_samples().iter().enumerate() {
                queue.push_back(*sample + pulse_samples[i] + pulse_sweep_samples[i]);
            }

            self.audio_queue_sender
                .send(AudioQueue {
                    queue,
                    shall_clear_old_samples: false,
                })
                .unwrap();
            self.pulse_sweep.reset_samples();
            self.pulse.reset_samples();
            self.wave.reset_samples();
        }
        self.pulse_sweep.tick(gb)?;
        self.pulse.tick(gb)?;
        self.wave.tick(gb)?;
        // self.noise.tick(gb);

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
            return Some(self.nr50);
        } else if addr == memory::apu::NR51 {
            return Some(self.nr51);
        } else if addr == memory::apu::NR52 {
            log::trace!("read nr52");
            return Some(self.nr52);
        }
        return None;
    }

    fn write8(&mut self, addr: u16, value: u8) -> Option<()> {
        if let Some(res) = self.pulse_sweep.write8(addr, value) {
        } else if let Some(res) = self.pulse.write8(addr, value) {
        } else if let Some(res) = self.wave.write8(addr, value) {
        } else if let Some(res) = self.noise.write8(addr, value) {
        } else if addr == memory::apu::NR50 {
            self.nr50 = value;
        } else if addr == memory::apu::NR51 {
            self.nr51 = value;
        } else if addr == memory::apu::NR52 {
            self.nr52 = value;
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

    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (tx, rx) = mpsc::channel();
        let mut apu = Self {
            pulse_sweep: pulse::PulseSweep::new(Self::AUDIO_SAMPLING_RATE),

            pulse: pulse::Pulse::new(Self::AUDIO_SAMPLING_RATE),

            wave: wave::Wave::new(Self::AUDIO_SAMPLING_RATE),

            noise: noise::Noise::new(),

            nr50: 0,
            nr51: 0,
            nr52: 0,

            div: 0,

            sink: Sink::try_new(&stream_handle).unwrap(),
            stream: _stream,
            stream_handle: stream_handle,
            audio_queue_sender: tx,
        };
        apu.sink.append(AudioDriver::new(Self::AUDIO_SAMPLING_RATE, 1, rx));
        apu
    }

    pub fn tick_div(&mut self) {
        self.div = self.div.wrapping_add(1);

        if self.div % APU::ENVELOPE_SWEEP_DIVIDER == 0 {
            // self.pulse_sweep.tick_envelope_sweep();
            // self.pulse.tick_envelope_sweep();
            // self.noise.tick_envelope_sweep();
        }

        if self.div % APU::SOUND_LENGTH_DIVIDER == 0 {
            self.pulse_sweep.tick_timer();
            self.pulse.tick_timer();
            self.wave.tick_timer();
            // self.noise.tick_timer();
        }

        if self.div % APU::CH1_FREQUENCY_SWEEP_DIVIDER == 0 {
            // self.pulse_sweep.tick_frequency_sweep();
        }
    }
}

trait APUChannel {
    fn tick_timer(&mut self);
    fn sample(&mut self);
    fn get_samples(&mut self) -> &Vec<f32>;
    fn reset_samples(&mut self);
    // fn get_current_sample(&self) -> u8; //unused right now (CGB)
    #[inline]
    fn dac(sample: u8, dac_enabled: bool) -> f32 {
        const SAMPLE_BIT_RESOLUTION: u8 = 16;
        if dac_enabled {
            // println!("dac result {}", sample as f32 / SAMPLE_BIT_RESOLUTION as f32);
            sample as f32 / SAMPLE_BIT_RESOLUTION as f32
        } else {
            0.0
        }
    }
}
