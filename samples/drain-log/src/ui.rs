//! ratatui TUI: live raw-log view + cluster table. No async runtime: one reader
//! thread feeds an `mpsc` channel; the main thread drains it in the draw loop.

use std::collections::VecDeque;
use std::io::Stdout;
use std::sync::mpsc::{self, Receiver};
use std::time::SystemTime;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEventKind};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use logdrain::{Cluster, Miner};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs};
use ratatui::{Frame, Terminal};

use crate::miner::{self, Stats};
use crate::source::{self, Input, Msg};

/// Raw lines kept in the ring buffer.
const RAW_CAP: usize = 10_000;
/// Throttle for the `miner.clusters()` snapshot.
const CLUSTER_REFRESH: Duration = Duration::from_millis(500);
/// Crossterm event poll tick.
const TICK: Duration = Duration::from_millis(100);

/// Enter the TUI and run until the user quits.
pub fn run(input: Input, miner: Miner) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<Msg>();
    source::install_ctrlc(tx.clone())?;
    let _reader = source::spawn_reader(input, tx.clone());
    drop(tx);

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new(miner, rx);
    let result = app.run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, event::DisableMouseCapture)?;
    result
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Raw,
    Clusters,
}

impl Tab {
    fn index(self) -> usize {
        self as usize
    }

    fn next(self) -> Self {
        match self {
            Tab::Raw => Tab::Clusters,
            Tab::Clusters => Tab::Raw,
        }
    }
}

pub struct App {
    miner: Miner,
    stats: Stats,
    rx: Receiver<Msg>,
    raw: VecDeque<String>,
    /// Offset from the newest raw line (0 = tail).
    raw_scroll: usize,
    clusters: Vec<Cluster>,
    cluster_idx: usize,
    active_tab: Tab,
    help: bool,
    input_done: bool,
    last_snapshot: Instant,
}

impl App {
    fn new(miner: Miner, rx: Receiver<Msg>) -> Self {
        App {
            miner,
            stats: Stats::default(),
            rx,
            raw: VecDeque::new(),
            raw_scroll: 0,
            clusters: Vec::new(),
            cluster_idx: 0,
            active_tab: Tab::Clusters,
            help: false,
            input_done: false,
            last_snapshot: Instant::now() - CLUSTER_REFRESH,
        }
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
        loop {
            let mut progressed = false;
            while let Ok(msg) = self.rx.try_recv() {
                match msg {
                    Msg::Line(line) => {
                        self.on_line(&line);
                        progressed = true;
                    }
                    Msg::Eof => self.input_done = true,
                    Msg::Stop => return Ok(()),
                }
            }

            if event::poll(TICK)? {
                match event::read()? {
                    Event::Key(key) => {
                        if self.on_key(key) {
                            return Ok(());
                        }
                    }
                    Event::Mouse(mouse) => match mouse.kind {
                        MouseEventKind::ScrollDown => self.scroll_down(),
                        MouseEventKind::ScrollUp => self.scroll_up(),
                        _ => {}
                    },
                    _ => {}
                }
            }

            self.refresh_clusters(progressed);
            terminal.draw(|frame| render(frame, self))?;
        }
    }

    fn on_line(&mut self, line: &str) {
        miner::ingest(&self.miner, &mut self.stats, line);
        if !line.trim().is_empty() {
            if self.raw.len() == RAW_CAP {
                self.raw.pop_front();
            }
            self.raw.push_back(line.to_owned());
        }
    }

    fn refresh_clusters(&mut self, force: bool) {
        let now = Instant::now();
        if force || now.duration_since(self.last_snapshot) >= CLUSTER_REFRESH {
            self.clusters = sorted_clusters(&self.miner.clusters());
            self.cluster_idx = self.cluster_idx.min(self.clusters.len().saturating_sub(1));
            self.last_snapshot = now;
        }
    }

    fn on_key(&mut self, key: KeyEvent) -> bool {
        if self.help {
            if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?')) {
                self.help = false;
            }
            return false;
        }
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Tab | KeyCode::BackTab => self.active_tab = self.active_tab.next(),
            KeyCode::Char('?') => self.help = true,
            KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
            KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
            KeyCode::Char('g') | KeyCode::Home => self.scroll_to_top(),
            KeyCode::Char('G') | KeyCode::End => self.scroll_to_bottom(),
            _ => {}
        }
        false
    }

    fn scroll_down(&mut self) {
        match self.active_tab {
            Tab::Raw => self.raw_scroll = self.raw_scroll.saturating_sub(1),
            Tab::Clusters if !self.clusters.is_empty() => {
                self.cluster_idx = (self.cluster_idx + 1).min(self.clusters.len() - 1);
            }
            Tab::Clusters => {}
        }
    }

    fn scroll_up(&mut self) {
        match self.active_tab {
            Tab::Raw => {
                let max = self.raw.len().saturating_sub(1);
                self.raw_scroll = (self.raw_scroll + 1).min(max);
            }
            Tab::Clusters => self.cluster_idx = self.cluster_idx.saturating_sub(1),
        }
    }

    fn scroll_to_top(&mut self) {
        match self.active_tab {
            Tab::Raw => self.raw_scroll = self.raw.len().saturating_sub(1),
            Tab::Clusters => self.cluster_idx = 0,
        }
    }

    fn scroll_to_bottom(&mut self) {
        match self.active_tab {
            Tab::Raw => self.raw_scroll = 0,
            Tab::Clusters if !self.clusters.is_empty() => {
                self.cluster_idx = self.clusters.len() - 1;
            }
            Tab::Clusters => {}
        }
    }
}

/// Clusters sorted by size (descending). Shared by the TUI and `--dump`.
pub fn sorted_clusters(clusters: &[Cluster]) -> Vec<Cluster> {
    let mut sorted = clusters.to_vec();
    sorted.sort_by_key(|c| std::cmp::Reverse(c.size()));
    sorted
}

fn render(frame: &mut Frame, app: &mut App) {
    let chunks =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)]).split(frame.area());
    render_header(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);
}

fn render_header(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let title = Line::from(vec![
        Span::styled(
            "drain-log",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
    ]);
    let tabs = Tabs::new(vec![Line::from(" Raw "), Line::from(" Clusters ")])
        .select(app.active_tab.index())
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let state = if app.input_done { "EOF" } else { "live" };
    let status = Line::from(vec![Span::styled(
        format!(
            "lines {}  clusters {}  with-ts {}  [{}]",
            app.stats.lines,
            app.clusters.len(),
            app.stats.with_timestamp,
            state
        ),
        Style::default().fg(Color::DarkGray),
    )]);

    let layout = Layout::horizontal([Constraint::Length(14), Constraint::Length(22), Constraint::Min(0)]).split(area);
    frame.render_widget(title, layout[0]);
    frame.render_widget(tabs, layout[1]);
    frame.render_widget(Paragraph::new(status), layout[2]);
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    if app.help {
        render_help(frame, area);
        return;
    }
    match app.active_tab {
        Tab::Raw => render_raw(frame, area, app),
        Tab::Clusters => render_clusters(frame, area, app),
    }
}

fn render_help(frame: &mut Frame, area: ratatui::layout::Rect) {
    let text = "q / Ctrl-C    quit\n\
                Tab           switch tab\n\
                j / k         scroll down / up (raw) or select (clusters)\n\
                g / G         jump to oldest / newest\n\
                mouse wheel   scroll\n\
                ?             toggle this help";
    frame.render_widget(
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Help ")),
        area,
    );
}

fn render_raw(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let n = app.raw.len();
    let height = area.height.saturating_sub(2) as usize;
    let end = n.saturating_sub(app.raw_scroll);
    let start = end.saturating_sub(height);
    let items: Vec<ListItem> = app
        .raw
        .range(start..end)
        .map(|line| ListItem::new(line.clone()))
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Raw ({n} buffered) ")),
    );
    frame.render_widget(list, area);
}

fn render_clusters(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let height = area.height.saturating_sub(2) as usize;
    let selected = app.cluster_idx;
    let offset = if selected >= height { selected - height + 1 } else { 0 };
    let rows: Vec<Row> = app
        .clusters
        .iter()
        .enumerate()
        .skip(offset)
        .take(height)
        .map(|(i, c)| {
            let style = if i == selected {
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Row::new(vec![
                Cell::from(c.id().to_string()),
                Cell::from(c.size().to_string()),
                Cell::from(fmt_lpm(c)),
                Cell::from(fmt_opt(c.event_first_seen())),
                Cell::from(fmt_opt(c.event_last_seen())),
                Cell::from(c.template().to_string()),
            ])
            .style(style)
        })
        .collect();
    let header = Row::new(vec![
        Cell::from("id"),
        Cell::from("size"),
        Cell::from("lpm"),
        Cell::from("first"),
        Cell::from("last"),
        Cell::from("template"),
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    let widths = [
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Length(22),
        Constraint::Length(22),
        Constraint::Min(10),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Clusters "));
    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let text = match app.active_tab {
        Tab::Clusters if !app.clusters.is_empty() => {
            let c = &app.clusters[app.cluster_idx];
            format!(
                "cluster {} size {}  first {}  last {}  {}",
                c.id(),
                c.size(),
                fmt_opt(c.event_first_seen()),
                fmt_opt(c.event_last_seen()),
                c.template()
            )
        }
        _ => "q quit | Tab switch | j/k scroll | ? help".to_string(),
    };
    frame.render_widget(Paragraph::new(text).style(Style::default().fg(Color::DarkGray)), area);
}

fn fmt_system_time(t: SystemTime) -> String {
    jiff::Timestamp::try_from(t)
        .map(|ts| ts.to_string())
        .unwrap_or_default()
}

fn fmt_opt(t: Option<SystemTime>) -> String {
    t.map(fmt_system_time).unwrap_or_else(|| "-".to_string())
}

fn fmt_lpm(c: &Cluster) -> String {
    c.event_lines_per_minute()
        .map(|v| format!("{v:.1}"))
        .unwrap_or_else(|| "-".to_string())
}
