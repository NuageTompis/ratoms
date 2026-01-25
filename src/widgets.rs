use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Widget},
};
use tui_big_text::{BigText, PixelSize};

use crate::{COLUMNS_AMOUNT, ratom::Ratom};

struct AtomCell {
    ratom: Ratom,
}

impl From<Ratom> for AtomCell {
    fn from(ratom: Ratom) -> Self {
        Self { ratom }
    }
}

impl Widget for AtomCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // construct the outer block
        let atom_block = Block::bordered()
            .gray()
            .title_alignment(Alignment::Right)
            .title_style(Style::default().bold())
            .title(format!(" {} ", self.ratom.get_number()));
        let atom_text_area = atom_block.inner(area);
        atom_block.render(area, buf);

        // construct the atomic symbol within the block
        let big_text = atom_big_text(self.ratom.get_symbol());
        big_text.render(atom_text_area, buf);
    }
}

#[derive(Debug)]
pub struct AtomicPeriod {
    pub number: u8,
    // for rows split in two like H - He
    pub left_row: Option<Vec<Ratom>>,
    pub right_row: Vec<Ratom>,
}

impl AtomicPeriod {
    pub fn new(number: u8) -> Self {
        Self {
            number,
            left_row: None,
            right_row: Vec::with_capacity(COLUMNS_AMOUNT),
        }
    }
}

impl Widget for AtomicPeriod {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width_constraints = [Constraint::Length(12); COLUMNS_AMOUNT];
        let layout = Layout::horizontal(width_constraints);
        let areas: [Rect; COLUMNS_AMOUNT] = layout.areas(area);

        // render the right part of the row
        for (atom, area) in (self.right_row.into_iter().rev()).zip(areas.into_iter().rev()) {
            let cell: AtomCell = atom.into();
            cell.render(area, buf);
        }

        // render the left part of the row
        if let Some(row) = self.left_row {
            for (atom, area) in row.into_iter().zip(areas) {
                let cell: AtomCell = atom.into();
                cell.render(area, buf);
            }
        }
    }
}

fn atom_big_text(symbol: &str) -> BigText<'_> {
    BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![symbol.bold().into()])
        .centered()
        .build()
}
