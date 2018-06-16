//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use super::*;
use Context;

use super::super::style;
use std::fmt;

/// Struct that stores an specific platform implementation for terminal related actions.
pub struct Terminal<'context> {
    terminal: Option<Box<ITerminal>>,
    context: &'context Context
}

impl<'context>  Terminal<'context> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new(context: &'context  Context) -> Terminal<'context> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal>>(WinApiTerminal::new(), AnsiTerminal::new());

        #[cfg(not(target_os = "windows"))]
        let terminal = Some(AnsiTerminal::new() as Box<ITerminal>);

        Terminal { terminal, context: context }

    }

    /// Clear the current cursor by specifying the clear type
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    /// 
    /// // clear all cells in terminal.
    /// term.clear(terminal::ClearType::All);
    /// // clear all cells from the cursor position downwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorDown);
    /// // clear all cells from the cursor position upwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(terminal::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(terminal::ClearType::UntilNewLine);
    /// 
    /// ```
    pub fn clear(&mut self, clear_type: ClearType) {
        if let Some(ref terminal) = self.terminal {
            terminal.clear(clear_type, &self.context);
        }
    }

    /// Get the terminal size (x,y).
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// 
    /// ```
    pub fn terminal_size(&mut self) -> (u16, u16) {
        if let Some(ref terminal) = self.terminal {
            return terminal.terminal_size(&self.context)
        }
        (0,0)
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    /// 
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// 
    /// ```
    pub fn scroll_up(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_up(count,&self.context);
        }
    }

    /// Scroll `n` lines up in the current terminal.
    /// 
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    /// 
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// 
    /// ```
    pub fn scroll_down(&mut self, count: i16) {
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_down(count, &self.context);
        }
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    /// use crossterm::terminal;
    /// use crossterm::Context;
    ///
    /// let context = Context::new();
    /// let mut term = terminal::terminal(&context);
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// 
    /// ```
    pub fn set_size(&mut self, width: i16, height: i16)
    {
        if let Some (ref terminal) = self.terminal {
            terminal.set_size(width,height,&self.context);
        }
    }

    /// Wraps an displayable object so it can be formatted with colors and attributes.
    ///
    /// Check `/examples/color` in the libary for more spesific examples.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::style::{paint,Color};
    ///
    /// fn main()
    /// {
    ///     // Create an styledobject object from the text 'Unstyled font'
    ///     // Currently it has the default foregroundcolor and backgroundcolor.
    ///     println!("{}",paint("Unstyled font"));
    ///
    ///     // Create an displayable object from the text 'Colored font',
    ///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
    ///     // Print the result.
    ///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
    ///     println!("{}", styledobject);
    ///
    ///     // Or all in one line
    ///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
    /// }
    /// ```
    pub fn paint<D>(&self, val: D) -> style::StyledObject<D>
        where
            D: fmt::Display,
    {
        style::ObjectStyle::new().apply_to(val, self.context.screen_manager.clone())
    }
}

/// Get an Terminal implementation whereon terminal related actions can be performed.
///
/// Check `/examples/version/terminal` in the libary for more spesific examples.
///
/// #Example
///
/// ```rust
///
/// extern crate crossterm;
/// use crossterm::terminal;
/// use crossterm::Context;
///
/// let context = Context::new();
///
/// let mut term = terminal::terminal(&context);
///
/// // scroll down by 5 lines
/// let size = term.scroll_down(5);
///
/// ```
///
pub fn terminal<'context>(context: &'context Context) -> Box<Terminal<'context>> {
    Box::from(Terminal::new(&context))
}