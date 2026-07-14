use sailfish::RenderError;
use sailfish::runtime::{Buffer, Render};
use std::fmt::{Display, Formatter};
use std::sync::Mutex;

pub struct State {
    pub selected: Mutex<Option<i64>>,
    pub duration: Mutex<Duration>,
}

#[derive(Copy, Clone)]
pub struct Duration(u8);

impl Duration {
    pub fn new(duration: u8) -> Option<Self> {
        if duration <= 21 {
            Some(Duration(duration))
        } else {
            None
        }
    }

    pub fn is_manual(&self) -> bool {
        self.0 == 21
    }

    pub fn get_duration(&self) -> u8 {
        self.0
    }
}

impl Render for Duration {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        self.0.render(b)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_manual() {
            write!(f, "Manuell")
        } else {
            write!(f, "{} sec", self.get_duration())
        }
    }
}
