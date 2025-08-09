// src/main.rs

mod music_generator;

use clap::Parser;
use midly::{Smf, MetaMessage, Timing, Format, Track, TrackEventKind};
use std::fs::File;
use std::io::Write;
use music_generator::{generate_random_notes, generate_markov_notes, Note};

/// Un generador de música algorítmica y disonante.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Número de notas a generar.
    #[arg(short, long, default_value_t = 100)]
    notes: u32,

    /// Nombre del archivo de salida MIDI.
    #[arg(short, long, default_value = "output.mid")]
    output: String,

    /// Modo de generación: 'random' o 'markov'.
    #[arg(short, long, default_value = "random")]
    mode: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let generated_notes: Vec<Note>;

    match args.mode.as_str() {
        "random" => {
            println!("Generando notas en modo aleatorio...");
            generated_notes = generate_random_notes(args.notes);
        },
        "markov" => {
            println!("Generando notas con Cadena de Markov...");
            let seed_pitches = [60, 61, 63, 66, 68, 71];
            generated_notes = generate_markov_notes(&seed_pitches, args.notes);
        },
        _ => {
            eprintln!("Error: El modo debe ser 'random' o 'markov'. Usando 'random' por defecto.");
            generated_notes = generate_random_notes(args.notes);
        }
    }

    let mut smf = Smf::new(midly::Header::new(
        Format::SingleTrack,
        Timing::Metrical(480.into()),
    ));

    let mut track = Track::new();

    for note in generated_notes {
        track.push(midly::TrackEvent {
            delta: note.duration.into(),
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: midly::MidiMessage::NoteOn {
                    key: note.pitch.into(),
                    vel: note.velocity.into(),
                },
            },
        });

        track.push(midly::TrackEvent {
            delta: note.duration.into(),
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: midly::MidiMessage::NoteOff {
                    key: note.pitch.into(),
                    vel: 0.into(),
                },
            },
        });
    }

    track.push(midly::TrackEvent {
        delta: 0.into(),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    smf.tracks.push(track);

    let mut buf = Vec::new();
    smf.write(&mut buf)?;

    let mut file = File::create(&args.output)?;
    file.write_all(&buf)?;

    println!("¡Generación completa! Abre el archivo '{}' con un reproductor MIDI.", args.output);

    Ok(())
}