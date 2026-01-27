use std::{fmt::Display, io};

use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEventKind},
    },
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{StatefulWidget, Widget},
};
use thiserror::Error;

use crate::{ratom::RatomBuildError, read_csv::read_csv_table_records};

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

fn handle_events(state: &mut AppState) -> io::Result<bool> {
    let should_exit = match crossterm::event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_press(state, key_event.code)
        }
        _ => false,
    };
    Ok(should_exit)
}

const ARROW_KEYCODES: [KeyCode; 4] = [KeyCode::Up, KeyCode::Right, KeyCode::Down, KeyCode::Left];
fn handle_key_press(state: &mut AppState, code: KeyCode) -> bool {
    // handle arrow movement
    if ARROW_KEYCODES.contains(&code) {
        match state.focused_cell {
            Some((mut i, mut j)) => {
                let within_matrix_bounds = !match code {
                    KeyCode::Up => overflowing_dec(&mut i),
                    KeyCode::Right => overflowing_inc(&mut j, COLUMNS_AMOUNT),
                    KeyCode::Down => overflowing_inc(&mut i, ROWS_AMOUNT),
                    KeyCode::Left => overflowing_dec(&mut j),
                    _ => panic!(),
                };
                if within_matrix_bounds && state.cells_matrix[i][j] {
                    state.focused_cell = Some((i, j));
                }
            }
            None => state.focused_cell = Some((0, 0)),
        }
    }

    // handle quit
    match code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if state.should_exit {
                return true;
            } else {
                state.should_exit = true;
            }
        }
        _ => (),
    }
    false
}

struct AppState {
    should_exit: bool,
    focused_cell: Option<(usize, usize)>,
    /// is *true* at index [i,j] if there is an atom rendered there
    cells_matrix: Vec<Vec<bool>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            should_exit: false,
            focused_cell: None,
            cells_matrix: Vec::new(),
        }
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
    let areas: [[Rect; COLUMNS_AMOUNT]; ROWS_AMOUNT] = areas.map(|area| {
        Layout::horizontal(
            [Constraint::Length(MIMIMUM_ATOMIC_CELL_DIMENSIONS.width); COLUMNS_AMOUNT],
        )
        .areas(area)
    });

    let cells_matrix = read_csv_table_records(state).unwrap();
    state.cells_matrix = cells_matrix
        .iter()
        .map(|row| row.iter().map(|option| option.is_some()).collect())
        .collect();

    for row in cells_matrix {
        for cell in row.into_iter().flatten() {
            let i = cell.row;
            let j = cell.column;
            cell.render(areas[i][j], buf);
        }
    }
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}

/// Decrerement index and informs if an overflow occured
fn overflowing_dec(index: &mut usize) -> bool {
    let res;
    (*index, res) = index.overflowing_sub(1);
    res
}

/// Increment index and informs if an overflow occured
///
/// assumes *index < upper_bound* and *upper_bound < usize::MAX*
fn overflowing_inc(index: &mut usize, upper_bound: usize) -> bool {
    *index += 1;
    *index == upper_bound
}
