use std::{fmt::Display, io};

use ratatui::{
    buffer::Buffer,
    crossterm,
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::{ratom::Ratom, widgets::AtomCell};

mod ratom;
mod widgets;

fn main() -> io::Result<()> {
    let mut state = AppState::new();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_stateful_widget(App, frame.area(), &mut state))?;
            if handle_events(&mut state)? {
                break Ok(());
            }
        }
    })
}

/// Return Ok(true) when the app should exit
fn handle_events(state: &mut AppState) -> io::Result<bool> {
    if crossterm::event::read()?.is_key_press() {
        if state.should_exit {
            return Ok(true);
        } else {
            state.should_exit = true;
        }
    }
    Ok(false)
}

struct AppState {
    should_exit: bool,
}

impl AppState {
    fn new() -> Self {
        Self { should_exit: false }
    }
}

struct App;

#[derive(Debug)]
struct Dimensions {
    width: u16,
    height: u16,
}

// we consider that the minimum requirements to render an element (including borders) is a square of 12x6
// with a display on 9 lines and 18 columns, this gives 216x54
const MINIMUM_DIMENSIONS: Dimensions = Dimensions {
    width: 216,
    height: 54,
};

impl From<Rect> for Dimensions {
    fn from(rect: Rect) -> Self {
        Self {
            width: rect.width,
            height: rect.height,
        }
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

fn test_sufficient_dimensions(area: Rect) -> Result<(), String> {
    let dimensions: Dimensions = area.into();
    if dimensions.height < MINIMUM_DIMENSIONS.height || dimensions.width < MINIMUM_DIMENSIONS.width
    {
        Err(format!(
            "insufficient dimensions: is {} but should be at least {}",
            dimensions, MINIMUM_DIMENSIONS
        ))
    } else {
        Ok(())
    }
}

impl StatefulWidget for App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.should_exit {
            let area = center_vertical(area, 1);
            Line::raw("press a key to confirm exit")
                .centered()
                .render(area, buf);
        } else {
            match test_sufficient_dimensions(area) {
                Ok(()) => {
                    render_table(area, buf, state);
                }
                Err(e) => {
                    let area = center_vertical(area, 1);
                    Line::raw(e).centered().render(area, buf);
                }
            }
        }
    }
}

fn render_table(area: Rect, buf: &mut Buffer, state: &mut AppState) {
    // minimal atom area (for testing purposes)
    let [area] = Layout::vertical([Constraint::Length(6)]).areas(area);
    let [area] = Layout::horizontal([Constraint::Length(12)]).areas(area);
    let ratom = Ratom::build(String::from("He"), 2, String::from("Helium")).unwrap();
    let atom_cell = AtomCell { ratom };
    atom_cell.render(area, buf);
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
