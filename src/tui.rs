use crate::util::Vec2;

use crossterm::{
    cursor,
    cursor::MoveTo,
    queue,
    style::{ContentStyle, Print, StyledContent},
    terminal::{self, disable_raw_mode, enable_raw_mode, size, Clear},
    ExecutableCommand,
};
use std::io::{self, Stdout, Write};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Window {
    pos: Vec2,
    size: Vec2,
    title: Option<String>,
    renderer: Rc<RefCell<Renderer>>,
}

#[allow(dead_code)]
impl Window {
    pub fn new(renderer: Rc<RefCell<Renderer>>, x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            pos: Vec2 { x, y },
            size: Vec2 {
                x: width,
                y: height,
            },
            title: None,
            renderer,
        }
    }

    pub fn centered(renderer: Rc<RefCell<Renderer>>, width: u16, height: u16) -> Self {
        let (x, y) = renderer.borrow().center_point();

        Window::new(renderer, x - width / 2, y - height / 2, width, height)
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn inner(&self) -> Self {
        Self {
            pos: Vec2 {
                x: self.pos.x + 1,
                y: self.pos.y + 1,
            },
            size: Vec2 {
                x: self.size.x - 2,
                y: self.size.y - 2,
            },
            title: None,
            renderer: self.renderer.clone(),
        }
    }

    pub fn outer(&self) -> Self {
        Self {
            pos: Vec2 {
                x: self.pos.x - 1,
                y: self.pos.y - 1,
            },
            size: Vec2 {
                x: self.size.x + 2,
                y: self.size.y + 2,
            },
            title: None,
            renderer: self.renderer.clone(),
        }
    }

    pub fn pixel(&self, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        self.renderer
            .borrow_mut()
            .pixel(self.pos.x + x, self.pos.y + y, c)?;
        Ok(())
    }

    pub fn pixel_styled(&self, x: u16, y: u16, c: StyledContent<char>) -> Result<(), io::Error> {
        self.renderer
            .borrow_mut()
            .pixel_styled(self.pos.x + x, self.pos.y + y, c)?;
        Ok(())
    }

    pub fn print_str(&self, x: u16, y: u16, s: &str) -> Result<(), io::Error> {
        self.renderer
            .borrow_mut()
            .print_str(x + self.pos.x, y + self.pos.y, s)?;
        Ok(())
    }

    pub fn print_centered_str(&self, y: u16, s: &str) -> Result<(), io::Error> {
        self.renderer.borrow_mut().print_str(
            self.size.x / 2 - (s.chars().count() / 2) as u16 + self.pos.x,
            y + self.pos.y,
            s,
        )?;
        Ok(())
    }

    pub fn draw_borders(&self) -> Result<(), io::Error> {
        for y in 1..self.size.y {
            self.pixel(0, y, '│')?;
            self.pixel(self.size.x, y, '│')?;
        }
        for x in 1..self.size.x {
            self.pixel(x, 0, '─')?;
            self.pixel(x, self.size.y, '─')?;
        }

        self.pixel(0, 0, '┌')?;
        self.pixel(self.size.x, 0, '┐')?;
        self.pixel(0, self.size.y, '└')?;
        self.pixel(self.size.x, self.size.y, '┘')?;

        if let Some(name) = &self.title {
            let title = format!("[ {} ]", name);
            self.print_centered_str(0, &title)?;
        }

        Ok(())
    }
}

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn center_point(&self) -> (u16, u16) {
        let (x, y) = size().unwrap();
        ((x / 2) as u16, (y / 2) as u16)
    }

    pub fn init(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        self.stdout.execute(terminal::EnterAlternateScreen)?;
        self.stdout.execute(cursor::Hide)?;

        Ok(())
    }

    pub fn dispose(&mut self) -> Result<(), io::Error> {
        self.stdout.execute(cursor::Show)?;
        self.stdout.execute(terminal::LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), io::Error> {
        queue!(self.stdout, Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn present(&mut self) -> Result<(), io::Error> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn pixel(&mut self, x: u16, y: u16, c: char) -> Result<(), io::Error> {
        self.pixel_styled(x, y, StyledContent::new(ContentStyle::new(), c))?;
        Ok(())
    }

    pub fn pixel_styled(
        &mut self,
        x: u16,
        y: u16,
        c: StyledContent<char>,
    ) -> Result<(), io::Error> {
        queue!(self.stdout, MoveTo(x, y), Print(&c))?;
        Ok(())
    }

    pub fn print_str(&mut self, x: u16, y: u16, s: &str) -> Result<(), io::Error> {
        queue!(self.stdout, MoveTo(x, y), Print(s))?;
        Ok(())
    }
}
