use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget},
};
use tui_big_text::{BigText, PixelSize};

use crate::ratom::Ratom;

pub struct AtomCell {
    pub ratom: Ratom,
}

impl Widget for AtomCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // construct the outer block
        let atom_block = Block::bordered().gray();
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
