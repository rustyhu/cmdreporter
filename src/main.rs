use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget, Wrap},
    DefaultTerminal,
};

mod perfcheck;

#[derive(Default)]
struct TabContents {
    cur_tab_idx: usize,
    reports: Vec<perfcheck::CmdOutput>,
}

impl TabContents {
    /// Move to the previous tab (loop move).
    fn previous(&mut self) {
        let l = self.reports.len();
        self.cur_tab_idx = (self.cur_tab_idx + l - 1) % l;
    }

    /// Move to the next tab, (loop).
    fn next(&mut self) {
        self.cur_tab_idx = (self.cur_tab_idx + 1) % self.reports.len();
    }

    const SELECTED_COLOR: Color = tailwind::GREEN.c900;
    /// orgnize all tab's name as tab title bar
    fn render_tabs_bar(&self, area: Rect, buf: &mut Buffer) {
        let titles = self.reports.iter().enumerate().map(|(idx, report)| {
            let cmdstr = report.cmdname.as_str();
            if idx == self.cur_tab_idx {
                cmdstr.bg(Self::SELECTED_COLOR).bold()
            } else {
                cmdstr.into()
            }
        });

        let highlight_style = (Color::default(), Self::SELECTED_COLOR);
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(self.cur_tab_idx)
            // .padding("", "")
            // .divider(" ")
            .render(area, buf);
    }
}

impl Widget for &TabContents {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // these might be separate widgets, currently only use block
        let showtext = self.reports[self.cur_tab_idx].summary.as_str();
        Paragraph::new(showtext)
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(TabContents::SELECTED_COLOR),
            )
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

#[derive(Default, PartialEq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

#[derive(Default)]
struct App {
    state: AppState,
    all_tabs: TabContents,
}

impl App {
    fn new(reports: Vec<perfcheck::CmdOutput>) -> Self {
        App {
            state: AppState::Running,
            all_tabs: TabContents {
                cur_tab_idx: 0,
                reports,
            },
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|f| f.render_widget(&self, f.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.all_tabs.next(),
                    KeyCode::Char('h') | KeyCode::Left => self.all_tabs.previous(),
                    KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::Quitting,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn render_note(&self, area: Rect, buf: &mut Buffer) {
        "Commands List".dim().render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("Press: [◄ ► / h l] to change tab | [q] to quit")
            .centered()
            .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, note_area] = horizontal.areas(header_area);

        // self.render_tabs(tabs_area, buf);
        self.all_tabs.render_tabs_bar(tabs_area, buf);
        self.render_note(note_area, buf);
        self.render_footer(footer_area, buf);
        self.all_tabs.render(inner_area, buf);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("Start to collect results of commands...");
    let cmd_outputs = perfcheck::collect()?;

    // TUI start
    let app = App::new(cmd_outputs);
    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
