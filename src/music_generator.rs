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

//pruebas unitarias

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_notes_count() {
        let count = 10;
        let notes = generate_random_notes(count);
        assert_eq!(notes.len() as u32, count);
    }

    #[test]
    fn test_generate_random_notes_duration_range() {
        let notes = generate_random_notes(1);
        let note = notes.first().unwrap();
        assert!(note.duration >= 100 && note.duration <= 500);
    }
    
 #[test]
 fn test_markov_chain_new_from_notes() {
     let seed_pitches = vec![60, 62, 64, 62, 60, 60];
     let chain = MarkovChain::new_from_notes(&seed_pitches);
     
     // Verifica las transiciones para la nota 60
     let transitions_60 = chain.transitions.get(&60).unwrap();
     assert_eq!(transitions_60, &[62, 60]);

     // Verifica las transiciones para la nota 62
     let transitions_62 = chain.transitions.get(&62).unwrap();
     assert_eq!(transitions_62, &[64, 60]);
     
     // Verifica las transiciones para la nota 64
     let transitions_64 = chain.transitions.get(&64).unwrap();
     assert_eq!(transitions_64, &[62]);
 }

    #[test]
    fn test_markov_chain_next_note_from_known_transition() {
        let seed_pitches = vec![60, 62, 64];
        let chain = MarkovChain::new_from_notes(&seed_pitches);
        let next_note = chain.next_note(60);
        assert_eq!(next_note, 62);
    }

    #[test]
    fn test_generate_markov_notes_count() {
        let seed_pitches = vec![60, 61, 62];
        let count = 5;
        let notes = generate_markov_notes(&seed_pitches, count);
        assert_eq!(notes.len() as u32, count);
    }
}