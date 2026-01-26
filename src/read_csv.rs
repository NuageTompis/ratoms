use serde::Deserialize;

use crate::{AppState, COLUMNS_AMOUNT, ROWS_AMOUNT, RatomsError, ratom::Ratom, widgets::AtomCell};

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

pub fn read_csv_table_records(state: &AppState) -> Result<Vec<Vec<Option<AtomCell>>>, RatomsError> {
    // initiate matrix of ROWS_AMOUNT x COLUMNS_AMOUNT
    let mut res: Vec<Vec<Option<AtomCell>>> = Vec::with_capacity(ROWS_AMOUNT);
    for _ in 0..ROWS_AMOUNT {
        let mut new_row = Vec::with_capacity(COLUMNS_AMOUNT);
        for _ in 0..COLUMNS_AMOUNT {
            new_row.push(None);
        }
        res.push(new_row);
    }

    let mut rdr = csv::Reader::from_path(CSV_PATH)?;
    for result in rdr.deserialize() {
        let record: ElementRecord = result?;
        let atom = Ratom::build(
            record.symbol.trim().to_string(),
            record.atomic_number,
            record.element,
        )?;

        if let Some(column) = record.group {
            let cell = AtomCell::new(atom, record.period as usize - 1, column as usize - 1);
            let (i, j) = (cell.row, cell.column);
            res[i][j] = Some(cell);
        }
    }

    if let Some((i, j)) = state.cell_highlighted {
        res[i][j]
            .as_mut()
            .expect("trying to highlight an empty cell")
            .highlighted = true;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_csv_table() {
        let mut rdr = csv::Reader::from_path(CSV_PATH).unwrap();
        for result in rdr.deserialize() {
            let record: ElementRecord = result.unwrap();
            let _ = Ratom::build(
                record.symbol.trim().to_string(),
                record.atomic_number,
                record.element,
            )
            .unwrap();
        }
    }
}
