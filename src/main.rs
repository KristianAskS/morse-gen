use clap::App;
use morse_gen::MorseAudioGenerator;

fn main() {
    let app = App::new("Morse Code Generator")
        .arg(
            clap::Arg::with_name("filename")
                .help("Filename to save audio to")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("text")
                .help("Text to generate audio from")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("wpm")
                .help("Words per minute")
                .required(false),
        );

    let matches = app.get_matches();
    let text = matches.value_of("text").unwrap();
    let filename = matches.value_of("filename").expect("No filename provided");
    let wpm = matches
        .value_of("wpm")
        .unwrap_or("10")
        .parse::<u32>()
        .unwrap();

    if !filename.ends_with(".wav") {
        panic!("Filename must end with .wav");
    }
    MorseAudioGenerator::new(text, filename, wpm).generate();
}
