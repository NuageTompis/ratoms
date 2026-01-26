use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    widgets::{Block, Widget},
};
use tui_big_text::{BigText, PixelSize};

use crate::ratom::Ratom;

pub struct AtomCell {
    pub ratom: Ratom,
    pub row: usize,
    pub column: usize,
}

impl AtomCell {
    pub fn new(ratom: Ratom, row: usize, column: usize) -> Self {
        Self { ratom, row, column }
    }
}

impl Widget for AtomCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // we temporarily don't render lanthanides and actinides (15 not 14)
        if (57_u8..=71).contains(&self.ratom.get_number())
            || (89_u8..=103).contains(&self.ratom.get_number())
        {
            return;
        }

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

fn atom_big_text(symbol: &str) -> BigText<'_> {
    BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![symbol.bold().into()])
        .centered()
        .build()
}
