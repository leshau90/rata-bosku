use std::io;

mod tui;
mod errors;



use ratatui::{
    buffer::Buffer,
    crossterm::event::{self,Event,KeyCode,KeyEvent,KeyEventKind},
    layout::{Alignment,Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block,Paragraph,Widget,
    },
    Frame,
};

use color_eyre::{
    eyre::{bail, Ok, WrapErr},
    Result,
};

#[derive(Debug,Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self,key_event: KeyEvent)-> Result<()>{
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self){
        self.exit = true;
    }

    fn increment_counter(&mut self) ->  Result<()>{
        self.counter += 1;
        if self.counter > 2 {
            bail!(format!("error: because {} is more than 2",self.counter));
        }
        Ok(())
    }

    fn decrement_counter(&mut self) ->  Result<()>{
        self.counter -= 1;
        Ok(())
    }

}



  

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer){
        let title = Title::from("Counter App by Ilman".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ])) ;      
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
               
            ).border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ])
        ]);

        Paragraph::new(counter_text)
            .centered().block(block).render(area, buf);

    }
}




fn main() -> Result<()>{
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    // let app_result = App::default().run(&mut terminal);
     App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())



    // app_result
}



#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert_eq!(app.exit, true);

    }
}

// use ratatui::{
//     backend::CrosstermBackend,
//     crossterm::{
//         event::{self, KeyCode, KeyEventKind},
//         terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//         ExecutableCommand,
//     },
//     style::Stylize,
//     widgets::Paragraph,
//     Terminal,
// };

// fn main() -> Result<()> {
//     stdout().execute(EnterAlternateScreen)?;
//     enable_raw_mode()?;
//     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//     terminal.clear()?;

//     loop {
//         terminal.draw(|frame| {
//             let area = frame.size();
//             frame.render_widget(
//                 Paragraph::new("Hello Ratatui! (press 'Q' to quit)")
//                     .white()
//                     .on_blue(),                
//                 area,
//             );
//         })?;



//         if event::poll(std::time::Duration::from_millis(16))? {
//             if let event::Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('Q') {
//                     break;
//                 }
//             }
//         }
//     }

//     stdout().execute(LeaveAlternateScreen)?;
//     disable_raw_mode()?;
//     Ok(())
// }




