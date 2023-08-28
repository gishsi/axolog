use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Record {
    /// todo: probably should not be pub, research getters and setters in Rust
    pub m_type: String, // todo: research String vs str
    pub cause: String,
    pub date: String,
    pub description: String,
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
}