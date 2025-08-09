use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

// Una estructura simple para representar una nota musical.
#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub pitch: u8,
    pub velocity: u8,
    pub duration: u32,
}

// Una escala disonante (ejemplo: tonos enteros)
const DISONANT_SCALE: [u8; 7] = [60, 62, 64, 66, 68, 70, 72];

/// Genera una secuencia de notas completamente aleatorias.
pub fn generate_random_notes(count: u32) -> Vec<Note> {
    let mut rng = rand::thread_rng();
    let mut notes = Vec::new();

    for _ in 0..count {
        let pitch = *DISONANT_SCALE.choose(&mut rng).unwrap();
        notes.push(Note {
            pitch,
            velocity: rng.gen_range(80..=120),
            duration: rng.gen_range(100..=500),
        });
    }

    notes
}

/// Una estructura para el modelo de la Cadena de Markov.
pub struct MarkovChain {
    transitions: HashMap<u8, Vec<u8>>,
}

impl MarkovChain {
    /// Crea una nueva cadena de Markov a partir de un conjunto de notas.
    pub fn new_from_notes(notes: &[u8]) -> Self {
        let mut transitions = HashMap::new();
        for i in 0..notes.len().saturating_sub(1) {
            transitions.entry(notes[i])
                       .or_insert_with(Vec::new)
                       .push(notes[i + 1]);
        }
        Self { transitions }
    }

    /// Genera la siguiente nota basándose en la nota actual.
    pub fn next_note(&self, current_note: u8) -> u8 {
        let mut rng = rand::thread_rng();
        if let Some(possible_next_notes) = self.transitions.get(&current_note) {
            *possible_next_notes.choose(&mut rng).unwrap_or(&current_note)
        } else {
            *DISONANT_SCALE.choose(&mut rng).unwrap_or(&60)
        }
    }
}

/// Genera notas usando una cadena de Markov.
pub fn generate_markov_notes(seed_pitches: &[u8], count: u32) -> Vec<Note> {
    let mut rng = rand::thread_rng();
    let markov_chain = MarkovChain::new_from_notes(seed_pitches);
    let mut notes = Vec::new();

    let mut current_pitch = *seed_pitches.first().unwrap_or(&DISONANT_SCALE[0]);
    
    for _ in 0..count {
        current_pitch = markov_chain.next_note(current_pitch);

        notes.push(Note {
            pitch: current_pitch,
            velocity: rng.gen_range(80..=120),
            duration: rng.gen_range(100..=500),
        });
    }

    notes
}