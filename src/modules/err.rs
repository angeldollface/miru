/*
MIRU by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct MiruErr {
    pub details: String
}

/// Implement generic methods.
impl MiruErr {

    /// Implements a generic method to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> MiruErr {
        MiruErr {
            details: details.to_owned()
        }
    }
}

/// Implements the "Error" trait.
impl Error for MiruErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the "Display" trait.
impl Display for MiruErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",self.details)
    }
}