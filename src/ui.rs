use ratatui::{
    prelude::{Line, Span, Text},
    style::{Color as TuiColor, Modifier, Style},
};

use crate::chess::{Board, Color, Position};

fn env_flag(name: &str) -> Option<bool> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .and_then(|value| match value.as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
}

pub fn prefers_unicode_symbols() -> bool {
    if let Some(false) = env_flag("CHESS_UNICODE") {
        return false;
    }

    if let Some(true) = env_flag("CHESS_ASCII") {
        return false;
    }

    true
}

fn piece_style(color: Color) -> Style {
    match color {
        Color::White => Style::default()
            .fg(TuiColor::Rgb(250, 250, 250))
            .add_modifier(Modifier::BOLD),
        Color::Black => Style::default().fg(TuiColor::Rgb(36, 28, 22)),
    }
}

pub fn board_text(
    board: &Board,
    cursor: Position,
    selected: Option<Position>,
    use_unicode_symbols: bool,
) -> Text<'static> {
    let mut lines = Vec::with_capacity(11);
    let files = (0..8)
        .map(|col| {
            Span::styled(
                format!(" {} ", (b'a' + col as u8) as char),
                Style::default().fg(TuiColor::Gray),
            )
        })
        .collect::<Vec<_>>();

    lines.push(Line::from(vec![Span::raw("   ")]));
    lines.push(Line::from(
        std::iter::once(Span::raw("   "))
            .chain(files.iter().cloned())
            .collect::<Vec<_>>(),
    ));

    for row in 0..8 {
        let mut line = Vec::with_capacity(10);
        let rank = 8 - row;
        line.push(Span::styled(
            format!(" {rank} "),
            Style::default().fg(TuiColor::Gray),
        ));

        for col in 0..8 {
            let position = Position::new(row, col);
            let mut square_style = if (row + col) % 2 == 0 {
                Style::default().bg(TuiColor::Rgb(240, 217, 181))
            } else {
                Style::default().bg(TuiColor::Rgb(181, 136, 99))
            };

            if Some(position) == selected {
                square_style = square_style
                    .bg(TuiColor::Rgb(120, 177, 89))
                    .add_modifier(Modifier::BOLD);
            }

            if position == cursor {
                square_style = square_style.add_modifier(Modifier::UNDERLINED | Modifier::REVERSED);
            }

            let span = match board.square(&position).piece() {
                Some(piece) => Span::styled(
                    format!(" {} ", piece.display_symbol(use_unicode_symbols)),
                    square_style.patch(piece_style(piece.color())),
                ),
                None => Span::styled("   ", square_style),
            };
            line.push(span);
        }

        line.push(Span::styled(
            format!(" {rank}"),
            Style::default().fg(TuiColor::Gray),
        ));
        lines.push(Line::from(line));
    }

    lines.push(Line::from(
        std::iter::once(Span::raw("   "))
            .chain(files)
            .collect::<Vec<_>>(),
    ));

    Text::from(lines)
}
