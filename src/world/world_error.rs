use std::fmt;
use std::error::Error;
use std::io;

use graphics::GraphicsError;

#[derive(Debug)]
pub enum WorldError {
    Graphics(GraphicsError)
}

impl From<GraphicsError> for WorldError {
    fn from(err: GraphicsError) -> Self {
        WorldError::Graphics(err)
    }
}

impl Error for WorldError {

    fn description(&self) -> &str {
        match *self {
            WorldError::Graphics(_) => "graphics",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WorldError::Graphics(ref err) => Some(err),
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}


