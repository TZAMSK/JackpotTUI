mod controle;
mod iu;

use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::crossterm::execute;
use ratatui::DefaultTerminal;

use controle::traiter_événement_clavier;

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
            afficher_machine(f, &size, size);
        })?;

        if let Err(_) = traiter_événement_clavier() {
            break Ok(());
        }
    }
}
