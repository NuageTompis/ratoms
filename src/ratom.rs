use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Ratom {
    symbol: String,
    number: u8,
    name: String,
    pub r#type: Option<Type>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Type {
    Lanthanide,
    Transactinide,
    Nonmetal,
    Metal,
    #[serde(rename = "Noble Gas")]
    NobleGas,
    Actinide,
    #[serde(rename = "Transition Metal")]
    TransitionMetal,
    Halogen,
    #[serde(rename = "Alkali Metal")]
    AlkaliMetal,
    Metalloid,
    #[serde(rename = "Alkaline Earth Metal")]
    AlkalineEarthMetal,
}

// increment this value if a new element is discovered
const ELEMENTS_COUNT: u8 = 118;

impl Ratom {
    /// Attempt to build a Ratom by checking that the symbol length and the atomic number given are in bounds
    pub fn build(
        symbol: String,
        number: u8,
        name: String,
        r#type: Option<Type>,
    ) -> Result<Self, RatomBuildError> {
        match symbol.len() {
            1 | 2 => {
                if (1..=ELEMENTS_COUNT).contains(&number) {
                    Ok(Self {
                        symbol,
                        number,
                        name,
                        r#type,
                    })
                } else {
                    Err(RatomBuildError::NumberOutOfBounds(number))
                }
            }
            len => Err(RatomBuildError::SymbolLengthOutOfBounds(len)),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RatomBuildError {
    #[error("symbol length is too long: found {0} should be in the range [1,2]")]
    SymbolLengthOutOfBounds(usize),
    #[error("atomic number does not exist: found {0} should be in the range [1-{ELEMENTS_COUNT}]")]
    NumberOutOfBounds(u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_atom() {
        // invalid entries
        Ratom::build(String::from(""), 1, String::from(""), None).unwrap_err();
        Ratom::build(String::from("Abc"), 1, String::from(""), None).unwrap_err();
        Ratom::build(String::from("H"), 0, String::from(""), None).unwrap_err();
        Ratom::build(String::from("H"), 119, String::from(""), None).unwrap_err();

        // valid entry
        Ratom::build(String::from("Fm"), 100, String::from("Fermium"), None).unwrap();
    }
}
