use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Record {
    m_type: String, 
    cause: String,
    date: String,
    description: String,
}

impl Record {
    pub fn _new(t: String, c: String, dt: String, desc: String, ) -> Record {
        Record {
            m_type: t,
            cause: c,
            date: dt,
            description: desc,
        }
    }

    /// Get the type of a record
    pub fn m_type(&self) -> &String {
        &self.m_type
    }

    /// Get the date of a record
    pub fn date(&self) -> &String {
        &self.date
    }

    /// Get the cause of a record
    pub fn cause(&self) -> &String {
        &self.cause
    }

    /// Set the description of a record
    pub fn description(&mut self) -> &mut String {
        &mut self.m_type
    }

    /// Get the description of a record
    pub fn get_description(&self) -> &String {
        &self.m_type
    }
}