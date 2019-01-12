use std::collections::HashMap;

pub struct Rna {
    amino_map: HashMap<&'static str, &'static str>,
}

impl Rna {
    pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
        match self.amino_map.get(codon) {
            Some(name) => Ok(name),
            None => Err("Invalid codon"),
        }
    }

    pub fn of_rna(&self, codon: &str) -> Result<Vec<&'static str>, &'static str> {
        let mut count = 0;
        let mut a_codon = String::new();
        let mut is_begining = true;
        let mut result_vec: Vec<&'static str> = Vec::new();

        for c in codon.chars() {
            if is_begining == true {
                a_codon += &c.to_string();
                is_begining = false;
                count += 1;
            } else {
                a_codon += &c.to_string();
                count += 1;

                if count == 3 {
                    match self.amino_map.get(&a_codon.as_ref()) {
                        Some(name) => result_vec.push(name),
                        None => break,
                    }
                    count = 0;
                    a_codon = "".to_string();

                    if result_vec[result_vec.len() - 1] == "stop codon" {
                        let end = &result_vec.len() - 1;
                        result_vec.truncate(end);
                        break;
                    }
                }
            }
        }

        if result_vec.len() < 1 {
            return Err("Invalid codon");
        } else {
            return Ok(result_vec);
        }
    }
}

pub fn parse(codon_list: Vec<(&'static str, &'static str)>) -> Rna {
    let mut rna_list = Rna {
        amino_map: HashMap::new(),
    };

    for (name, codons) in codon_list.into_iter() {
        rna_list.amino_map.insert(name, codons);
    }

    rna_list
}
