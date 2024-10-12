use color_eyre::Result;
use perfcheck::Report;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
    DefaultTerminal,
};

mod perfcheck;

#[derive(Default)]
struct TabContents {
    cur_tab_idx: usize,

    reports: Vec<perfcheck::Report>,
}

impl TabContents {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(&mut self) {
        if self.cur_tab_idx == 0 {
            return;
        }
        self.cur_tab_idx -= 1;
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(&mut self) {
        if self.cur_tab_idx == self.reports.len() - 1 {
            return;
        }
        self.cur_tab_idx += 1;
    }

    /// orgnize all tab's name as tab title bar
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let titles = self.reports.iter().enumerate().map(|(idx, report)| {
            report
                .cmdname
                .clone()
                .fg(tailwind::SLATE.c200)
                .bg(self.palette(idx).c900)
        });

        let highlight_style = (Color::default(), tailwind::GREEN.c700);
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(self.cur_tab_idx)
            // .padding("", "")
            // .divider(" ")
            .render(area, buf);
    }

    const fn palette(&self, idx: usize) -> tailwind::Palette {
        match idx == self.cur_tab_idx {
            true => tailwind::GREEN,
            _ => tailwind::GRAY,
        }
    }
}

impl Widget for &TabContents {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // clear first
        // Clear.render(area, buf);

        // these might be separate widgets, currently only use block
        let showtext = self.reports[self.cur_tab_idx].summary.as_str();
        Paragraph::new(showtext)
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(self.palette(self.cur_tab_idx).c700),
            )
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
    fn new(reports: Vec<Report>) -> Self {
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
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.all_tabs.next();
    }

    pub fn previous_tab(&mut self) {
        self.all_tabs.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

fn render_note(area: Rect, buf: &mut Buffer) {
    "Perf 60s cmds list".dim().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("[◄ ► / h l] to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, note_area] = horizontal.areas(header_area);

        // self.render_tabs(tabs_area, buf);
        self.all_tabs.render_title_bar(tabs_area, buf);
        render_note(note_area, buf);
        render_footer(footer_area, buf);
        self.all_tabs.render(inner_area, buf);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Start to collect results of perf checks...");
    let reports = perfcheck::collect();
    let app = App::new(reports);

    let terminal = ratatui::init();
    let app_result = app.run(terminal);
    ratatui::restore();
    app_result
}
