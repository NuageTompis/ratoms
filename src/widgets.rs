use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::border::{PLAIN, ROUNDED, THICK},
    widgets::{Block, Widget},
};
use tui_big_text::{BigText, PixelSize};

use crate::ratom::Ratom;

const FOCUSED_STYLE: Style = Style {
    fg: Some(Color::Cyan),
    bg: None,
    underline_color: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::from_bits(0_u16).unwrap(),
};
const DEFAULT_STYLE: Style = Style {
    fg: None,
    bg: None,
    underline_color: None,
    add_modifier: Modifier::from_bits(0_u16).unwrap(),
    sub_modifier: Modifier::from_bits(0_u16).unwrap(),
};

const fn get_focus_style(focused: bool) -> Style {
    if focused {
        FOCUSED_STYLE
    } else {
        DEFAULT_STYLE
    }
}

pub struct AtomCell {
    pub ratom: Ratom,
    pub focused: bool,
}

impl AtomCell {
    pub fn new(ratom: Ratom, focused: bool) -> Self {
        Self { ratom, focused }
    }
}

impl Widget for AtomCell {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // construct the outer block
        let atom_block = Block::bordered()
            .border_set(if self.focused { THICK } else { PLAIN })
            .gray()
            .style(get_focus_style(self.focused))
            .title_alignment(Alignment::Right)
            .title_style(Style::default().bold())
            .title(format!(" {} ", self.ratom.get_number()));
        let atom_text_area = atom_block.inner(area);
        atom_block.render(area, buf);

        // construct the atomic symbol within the block
        let mut big_text = atom_big_text(self.ratom.get_symbol());
        big_text.alignment = Alignment::Center;
        big_text.render(atom_text_area, buf);
    }
}

pub fn atom_big_text(symbol: &str) -> BigText<'_> {
    BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![symbol.bold().into()])
        .style(Style::default().white())
        .build()
}

pub enum InfoBlockState {
    ElementDescription,
    Groups,
}

pub struct InformationBlock {
    state: InfoBlockState,
    focused_atom: Option<Ratom>,
    focused: bool,
}

impl InformationBlock {
    pub fn new(state: InfoBlockState, focused_atom: Option<Ratom>, focused: bool) -> Self {
        Self {
            state,
            focused_atom,
            focused,
        }
    }
}

impl Widget for InformationBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (title, big_text) = if let Some(atom) = &self.focused_atom {
            let name = atom.get_name();
            (name, Some(atom_big_text(name)))
        } else {
            ("information block", None)
        };
        let atom_block = Block::bordered()
            .border_set(ROUNDED)
            .gray()
            .title(title)
            .border_style(get_focus_style(self.focused));
        atom_block.render(area, buf);

        if let Some(big_text) = big_text {
            big_text.render(area.inner(Margin::new(2, 1)), buf);
        }
    }
}
