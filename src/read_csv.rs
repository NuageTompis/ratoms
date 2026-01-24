use std::io;

use serde::Deserialize;

use crate::{ROWS_AMOUNT, RatomsError, ratom::Ratom, widgets::AtomicPeriod};

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
    group: String,
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

pub fn read_periods() -> Result<Vec<AtomicPeriod>, RatomsError> {
    let mut res: Vec<AtomicPeriod> = (1..ROWS_AMOUNT)
        .map(|row| AtomicPeriod::new(row as u8))
        .collect();

    let mut rdr = csv::Reader::from_path(CSV_PATH)?;
    for result in rdr.deserialize() {
        let record: ElementRecord = result?;
        let atom = Ratom::build(
            record.symbol.trim().to_string(),
            record.atomic_number,
            record.element,
        )?;
        res[record.period as usize - 1].right_row.push(atom);
    }
    Ok(res)
}
