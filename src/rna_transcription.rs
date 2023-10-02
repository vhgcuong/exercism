#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const DNA: [char; 4] = ['G', 'C', 'T', 'A'];
const RNA: [char; 4] = ['G', 'C', 'U', 'A'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|ch| !DNA.contains(&ch)) {
            None => Ok(Dna(dna.to_owned())),
            Some(x) => Err(x)
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars()
            .map(|ch| match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => ch
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|ch| !RNA.contains(&ch)) {
            None => Ok(Rna(rna.to_owned())),
            Some(x) => Err(x)
        }
    }
}
