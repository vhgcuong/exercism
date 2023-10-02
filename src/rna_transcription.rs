#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides: Vec<char> = vec!['G', 'C', 'T', 'A'];
        if dna.chars().all(|ch| nucleotides.contains(&ch)) {
            return Ok(Dna(dna.to_string()));
        }

        if let Some(position) = dna.chars().position(|ch| !nucleotides.contains(&ch)) {
            return Err(position);
        }

        Err(0)
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.0.chars().map(|ch| match ch {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => '_'
        }).collect::<String>();

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides: Vec<char> = vec!['G', 'C', 'U', 'A'];
        if rna.chars().all(|ch| nucleotides.contains(&ch)) {
            return Ok(Rna(rna.to_string()));
        }

        if let Some(position) = rna.chars().position(|ch| !nucleotides.contains(&ch)) {
            return Err(position);
        }

        Err(0)
    }
}
