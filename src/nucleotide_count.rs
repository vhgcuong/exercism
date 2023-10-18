use std::collections::HashMap;

const DNA: [char; 4] = ['G', 'C', 'T', 'A'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for ch in dna.chars() {
        if !DNA.contains(&ch) {
            return Err(ch);
        }

        if ch == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result: HashMap<char, usize> = HashMap::from([('A', 0), ('T', 0), ('C', 0), ('G', 0)]);

    for ch in dna.chars() {
        if !DNA.contains(&ch) {
            return Err(ch);
        }

        *result.entry(ch).or_default() += 1;
    }

    Ok(result)
}
