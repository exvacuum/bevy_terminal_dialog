//! bevy_terminal_display widgets for dialog boxes

use bevy::prelude::*;
use bevy_terminal_display::{crossterm, ratatui::{layout::{Alignment, Constraint, Flex, Layout}, style::Style, text::{Line, Span}, widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Wrap}}, widgets::TerminalWidget};
use yarnspinner::runtime::DialogueOption;
use unicode_segmentation::UnicodeSegmentation as _;
use arbitrary_chunks::ArbitraryChunks as _;

/// Interaction tooltip widget marker
// TODO: Move tooltip out of this crate?
#[derive(Component)]
pub struct InteractTooltip;

/// Interaction tooltip widget
pub struct InteractTooltipWidget;

impl TerminalWidget for InteractTooltipWidget {
    fn render(
        &mut self,
        frame: &mut bevy_terminal_display::ratatui::Frame,
        rect: bevy_terminal_display::ratatui::prelude::Rect,
    ) {
        let text = Paragraph::new("E")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1)),
            )
            .alignment(Alignment::Center);
        let [area] = Layout::horizontal([Constraint::Length(5)])
            .flex(Flex::Center)
            .areas(rect);
        let [area] = Layout::vertical([Constraint::Length(3)])
            .flex(Flex::Center)
            .areas(area);
        frame.render_widget(Clear, area);
        frame.render_widget(text, area);
    }
}

/// Dialog box widget marker
#[derive(Component)]
pub struct DialogBox;

/// Dialog box widget
#[derive(Default)]
pub struct DialogBoxWidget {
    /// Name of speaking character
    pub character: Option<String>,
    /// Chunks of text and corresponding styles representing the currently spoken line
    pub text: Vec<(String, Style)>,
}

impl TerminalWidget for DialogBoxWidget {
    fn render(
        &mut self,
        frame: &mut bevy_terminal_display::ratatui::Frame,
        rect: bevy_terminal_display::ratatui::prelude::Rect,
    ) {
        let text = Paragraph::new(bevy_terminal_display::ratatui::text::Line::from(
            self.text
                .iter()
                .map(|(text, style)| Span::styled(text, *style))
                .collect::<Vec<_>>(),
        ))
        .wrap(Wrap { trim: true })
        .block({
            let mut block = Block::new()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1));
            if let Some(character) = &self.character {
                block = block.title(character.clone());
            }
            block
        });
        let [area] = Layout::horizontal([Constraint::Max(100)])
            .flex(Flex::Center)
            .areas(rect);
        let [area] = Layout::vertical([Constraint::Max(10)])
            .flex(Flex::End)
            .areas(area);
        frame.render_widget(Clear, area);
        frame.render_widget(text, area);
    }
}

/// Option selection box widget marker
#[derive(Component)]
pub struct OptionsBox;

/// Option selection box widget
#[derive(Default)]
pub struct OptionsBoxWidget {
    /// State of ratatui list widget
    pub state: ListState,
    /// Available dialog options
    pub options: Vec<(DialogueOption, Vec<(String, Style)>)>,
}

impl TerminalWidget for OptionsBoxWidget {
    fn render(
        &mut self,
        frame: &mut bevy_terminal_display::ratatui::Frame,
        rect: bevy_terminal_display::ratatui::prelude::Rect,
    ) {
        let terminal_size = crossterm::terminal::size().unwrap();
        let box_size: u16 = if terminal_size.0 > 40 { 20 } else { 10 };
        let items = self
            .options
            .iter()
            .map(|option| {
                let option_spans = option
                    .1
                    .iter()
                    .map(|(text, style)| Span::styled(text, *style))
                    .collect::<Vec<_>>();
                let string_chunks = textwrap::wrap(
                    &Line::from(option_spans.clone()).to_string(),
                    textwrap::Options::new(box_size as usize),
                )
                .into_iter()
                .map(|chunk| chunk.graphemes(true).count())
                .collect::<Vec<_>>();

                let line = Line::from(option_spans);
                let graphemes = line.styled_graphemes(Style::default()).collect::<Vec<_>>();

                let mut final_string_chunks = vec![];
                let mut i = 0;
                for string_chunk in string_chunks {
                    final_string_chunks.push(string_chunk);
                    i += string_chunk;
                    if let Some(grapheme) = graphemes.get(i) {
                        if grapheme.symbol == " " {
                            final_string_chunks.push(1);
                            i += 1;
                        }
                    }
                }

                let mut chunked_graphemes = graphemes
                    .arbitrary_chunks(&final_string_chunks)
                    .collect::<Vec<_>>();
                chunked_graphemes.retain(|chunk| !(chunk.len() == 1 && chunk[0].symbol == " "));

                let line = chunked_graphemes
                    .into_iter()
                    .map(|chunk| {
                        Line::from(
                            chunk
                                .iter()
                                .map(|grapheme| {
                                    Span::styled(grapheme.symbol.to_string(), grapheme.style)
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>();
                ListItem::new(line)
            })
            .collect::<Vec<_>>();

        let height = items.iter().fold(0, |acc, item| acc + item.height());

        let outer_block = Block::bordered().padding(Padding::horizontal(1));
        let list = List::new(items)
            .block(outer_block)
            .highlight_symbol("-> ")
            .highlight_spacing(HighlightSpacing::Always);

        let [area] = Layout::horizontal([Constraint::Length(box_size + 7)])
            .flex(Flex::Center)
            .areas(rect);
        let [area] = Layout::vertical([Constraint::Length(height as u16 + 2)])
            .flex(Flex::Center)
            .areas(area);

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(list, area, &mut self.state);
    }
}