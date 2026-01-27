use serde::Deserialize;

use crate::{COLUMNS_AMOUNT, ROWS_AMOUNT, RatomsError, ratom::Ratom};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct ElementRecord {
    atomic_number: u8,
    element: String,
    symbol: String,
    atomic_mass: String,
    numberof_neutrons: String,
    numberof_protons: String,
    numberof_electrons: String,
    period: u8,
    group: Option<u8>,
    phase: String,
    radioactive: String,
    natural: String,
    metal: String,
    nonmetal: String,
    metalloid: String,
    r#type: String,
    atomic_radius: String,
    electronegativity: String,
    first_ionization: String,
    density: String,
    melting_point: String,
    boiling_point: String,
    number_of_isotopes: String,
    discoverer: String,
    year: String,
    specific_heat: String,
    numberof_shells: String,
    numberof_valence: String,
}

const CSV_PATH: &str = "resources/periodic-table-of-elements.csv";

pub fn read_csv_table_records() -> Result<Vec<Vec<Option<Ratom>>>, RatomsError> {
    // initiate matrix of ROWS_AMOUNT x COLUMNS_AMOUNT
    let mut res: Vec<Vec<Option<Ratom>>> = vec![vec![None; COLUMNS_AMOUNT]; ROWS_AMOUNT];

    let mut rdr = csv::Reader::from_path(CSV_PATH)?;
    for result in rdr.deserialize() {
        let record: ElementRecord = result?;
        let atom = Ratom::build(
            record.symbol.trim().to_string(),
            record.atomic_number,
            record.element,
        )?;

        if let Some(column) = record.group {
            let (i, j) = (record.period as usize - 1, column as usize - 1);
            res[i][j] = Some(atom);
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_csv_table() {
        read_csv_table_records().unwrap();
    }
}
