use hound::{self, WavWriter};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufWriter;

const AMPLITUDE: f32 = i16::MAX as f32;

const HERTZ: f32 = 800.0;

const MORSE_CODE_MAP: [(char, &str); 55] = [
    ('a', ".-"),
    ('b', "-..."),
    ('c', "-.-."),
    ('d', "-.."),
    ('e', "."),
    ('f', "..-."),
    ('g', "--."),
    ('h', "...."),
    ('i', ".."),
    ('j', ".---"),
    ('k', "-.-"),
    ('l', ".-.."),
    ('m', "--"),
    ('n', "-."),
    ('o', "---"),
    ('p', ".--."),
    ('q', "--.-"),
    ('r', ".-."),
    ('s', "..."),
    ('t', "-"),
    ('u', "..-"),
    ('v', "...-"),
    ('w', ".--"),
    ('x', "-..-"),
    ('y', "-.--"),
    ('z', "--.."),
    (' ', "/"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
    ('0', "-----"),
    ('.', ".-.-.-"),
    (',', "--..--"),
    ('?', "..--.."),
    ('\'', ".----."),
    ('!', "-.-.--"),
    ('/', "-..-."),
    ('(', "-.--."),
    (')', "-.--.-"),
    ('&', ".-..."),
    (':', "---..."),
    (';', "-.-.-."),
    ('=', "-...-"),
    ('+', ".-.-."),
    ('-', "-....-"),
    ('_', "..--.-"),
    ('"', ".-..-."),
    ('$', "...-..-"),
    ('@', ".--.-."),
];

pub struct MorseAudioGenerator {
    morse_code_text: String,
    writer: RefCell<WavWriter<BufWriter<File>>>,
    unit_freq_length: u32,
}

impl MorseAudioGenerator {
    pub fn new(text: &str, filename: &str, wpm: u32) -> Self {
        let spec = RefCell::new(hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        });
        let writer = RefCell::new(WavWriter::create(filename, spec.borrow().clone()).unwrap());
        MorseAudioGenerator {
            morse_code_text: text_to_morse(text.to_lowercase().as_str()),
            writer,
            unit_freq_length: (44100.0 * wpm_to_unit_length(wpm)) as u32,
        }
    }

    pub fn generate(&self) {
        for (word_index, word) in self.morse_code_text.split("/").enumerate() {
            for char in word.split(" ") {
                self.add_char(char);
                self.add_inter_char_space();
            }
            if word_index < self.morse_code_text.split("/").count() - 1 {
                self.add_word_space();
            }
        }
    }

    fn add_char(&self, morse_char: &str) {
        for (i, c) in morse_char.chars().enumerate() {
            if c == '.' {
                self.add_dit();
            } else if c == '-' {
                self.add_dah();
            }

            if i < morse_char.len() - 1 {
                self.add_intra_char_space();
            }
        }
    }

    fn add_dit(&self) {
        let mut writer = self.writer.borrow_mut();
        for t in (0..self.unit_freq_length).map(|x| x as f32 / 44100.0) {
            let sample = (t * HERTZ * 2.0 * PI).sin();
            writer.write_sample((sample * AMPLITUDE) as i16).unwrap();
        }
    }

    fn add_dah(&self) {
        let mut writer = self.writer.borrow_mut();
        for t in (0..self.unit_freq_length * 3).map(|x| x as f32 / 44100.0) {
            let sample = (t * HERTZ * 2.0 * PI).sin();
            writer.write_sample((sample * AMPLITUDE) as i16).unwrap();
        }
    }

    fn add_inter_char_space(&self) {
        let mut writer = self.writer.borrow_mut();
        for _ in 0..self.unit_freq_length * 3 {
            writer.write_sample(0 as i16).unwrap();
        }
    }

    fn add_intra_char_space(&self) {
        let mut writer = self.writer.borrow_mut();
        for _ in 0..self.unit_freq_length {
            writer.write_sample(0 as i16).unwrap();
        }
    }

    fn add_word_space(&self) {
        let mut writer = self.writer.borrow_mut();
        for _ in 0..self.unit_freq_length * 4 {
            writer.write_sample(0 as i16).unwrap();
        }
    }
}

fn text_to_morse(text: &str) -> String {
    text.chars()
        .map(|c| {
            match MORSE_CODE_MAP.iter().find(|&(letter, _)| letter == &c) {
                Some(val) => val,
                None => {
                    panic!("No morse code for '{}' implemented.", c)
                }
            }
            .1
        })
        .collect::<Vec<&str>>()
        .join(" ")
        .replace(" / ", "/")
}

fn wpm_to_unit_length(wpm: u32) -> f32 {
    60.0 / (wpm * 50) as f32
}
