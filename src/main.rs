use std::{fmt::Display, io};

use ratatui::{
    buffer::Buffer,
    crossterm,
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{StatefulWidget, Widget},
};
use thiserror::Error;

use crate::{ratom::RatomBuildError, read_csv::read_periods};

mod ratom;
mod read_csv;
mod widgets;

#[derive(Error, Debug)]
pub enum RatomsError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("ratom build error")]
    RatomBuild(#[from] RatomBuildError),
    #[error("csv error")]
    Csv(#[from] csv::Error),
}

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
const MIMIMUM_ATOMIC_CELL_DIMENSIONS: Dimensions = Dimensions {
    width: 12,
    height: 6,
};

const ROWS_AMOUNT: usize = 9;
const COLUMNS_AMOUNT: usize = 18;

// with a display on 9 lines and 18 columns, this gives 216x54
const MINIMUM_WINDOW_DIMENSIONS: Dimensions = Dimensions {
    width: COLUMNS_AMOUNT as u16 * MIMIMUM_ATOMIC_CELL_DIMENSIONS.width,
    height: ROWS_AMOUNT as u16 * MIMIMUM_ATOMIC_CELL_DIMENSIONS.height,
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
    if dimensions.height < MINIMUM_WINDOW_DIMENSIONS.height
        || dimensions.width < MINIMUM_WINDOW_DIMENSIONS.width
    {
        Err(format!(
            "insufficient dimensions: is {} but should be at least {}",
            dimensions, MINIMUM_WINDOW_DIMENSIONS
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
    let areas: [Rect; ROWS_AMOUNT] =
        Layout::vertical([Constraint::Length(MIMIMUM_ATOMIC_CELL_DIMENSIONS.height); ROWS_AMOUNT])
            .areas(area);
    let periods = read_periods().unwrap();
    for (period, area) in periods.into_iter().zip(areas) {
        period.render(area, buf);
    }
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
