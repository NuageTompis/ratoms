use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Widget},
};
use tui_big_text::{BigText, PixelSize};

use crate::ratom::Ratom;

pub fn atom_cell(area: Rect, buf: &mut Buffer, atom: Ratom) {
    let atom_block = Block::bordered().gray();
    let atom_text_area = atom_block.inner(area);
    atom_block.render(area, buf);

    let big_text = atom_big_text(atom.get_symbol());
    big_text.render(atom_text_area, buf);
}

fn atom_big_text(symbol: &str) -> BigText<'_> {
    BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![symbol.bold().into()])
        .centered()
        .build()
}
