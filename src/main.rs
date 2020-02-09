use std::convert::TryFrom;

#[derive(Debug)]
enum Note { C, Db, D, Eb, E, F, Gb, G, Ab, A, Bb, B }

impl TryFrom<String> for Note {
    type Error = &'static str;

    fn try_from(note: String) -> Result<Note, Self::Error> {
        match note.as_str() {
            "C" => Ok(Note::C),
            "Db" => Ok(Note::Db),
            "D" => Ok(Note::D),
            "Eb" => Ok(Note::Eb),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "Gb" => Ok(Note::Gb),
            "G" => Ok(Note::G),
            "Ab" => Ok(Note::Ab),
            "A" => Ok(Note::A),
            "Bb" => Ok(Note::Bb),
            "B" => Ok(Note::B),
            _ => Err("Unknown"),
        }
    }
}

type Pitch = fn(&Note) -> Note;

fn semitone(note: &Note) -> Note {
    match note {
        Note::C => Note::Db,
        Note::Db => Note::D,
        Note::D => Note::Eb,
        Note::Eb => Note::E,
        Note::E => Note::F,
        Note::F => Note::Gb,
        Note::Gb => Note::G,
        Note::G => Note::Ab,
        Note::Ab => Note::A,
        Note::A => Note::Bb,
        Note::Bb => Note::B,
        Note::B => Note::C,
    }
}

fn tone(note: &Note) -> Note {
    match note {
        Note::C => Note::D,
        Note::Db => Note::Eb,
        Note::D => Note::E,
        Note::Eb => Note::F,
        Note::E => Note::Gb,
        Note::F => Note::G,
        Note::Gb => Note::Ab,
        Note::G => Note::A,
        Note::Ab => Note::Bb,
        Note::A => Note::B,
        Note::Bb => Note::C,
        Note::B => Note::Db,
    }
}

lazy_static::lazy_static! {
    static ref MAJOR: Vec<Pitch> = vec![tone, tone, semitone, tone, tone, tone, semitone];
}

fn eval(tonic: Note, scale: Vec<Pitch>) -> Vec<Note> {
    scale.iter()
        .fold(vec![tonic], move |mut notes, pitch| {
            notes.push(pitch(notes.last().unwrap()));
            notes
        })
}

fn main() {
    let mut args = std::env::args();
    let tonic = args.nth(1).unwrap();
    let tonic = Note::try_from(tonic).unwrap_or(Note::C);

    let result = eval(tonic, MAJOR.to_vec());

    println!("{:?}", result);
}
