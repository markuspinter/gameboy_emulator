use std::{sync::mpsc, time::Duration};

use rodio::{Sample, Source};

pub type Receiver = mpsc::Receiver<(f32, f32)>;
pub type Sender = mpsc::Sender<(f32, f32)>;

pub trait CustomSource<T>: Source + Iterator
where
    <Self as Iterator>::Item: rodio::Sample,
{
    fn set_parameters(&mut self, parameters: T);
}

// now returns the channel's sender ADN the Speed object
pub fn speed<I, T>(input: I) -> (Speed<I, T>, mpsc::Sender<T>) {
    let (tx, rx) = mpsc::channel();
    (Speed { input, rx }, tx)
}

// now includes the channel's receiver
pub struct Speed<I, T> {
    input: I,
    rx: mpsc::Receiver<T>,
}

impl<I, T> Source for Speed<I, T>
where
    I: CustomSource<T>,
    I::Item: Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.input.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}

impl<I, T> Iterator for Speed<I, T>
where
    I: CustomSource<T>,
    I::Item: Sample,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        // check for channel update
        match self.rx.try_recv() {
            Ok(params) => self.input.set_parameters(params),
            _ => (),
        }
        self.input.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}
