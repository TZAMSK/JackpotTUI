mod controle;
mod iu;

use color_eyre::Result;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::execute;
use ratatui::DefaultTerminal;

use controle::traiter_événement_clavier;
use iu::iu_machine::afficher_machine;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen);

    let mut terminal = ratatui::init();
    let resultat = run(&mut terminal);
    ratatui::restore();

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    resultat
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            afficher_machine(f, size);
        })?;

        if let Err(_) = traiter_événement_clavier() {
            break Ok(());
        }
    }
}
