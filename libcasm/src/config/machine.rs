use serde::{Deserialize, Serialize};
use crate::config::errors::Error as MachineError;
use std::io::Read;

#[derive(Deserialize, Serialize, Debug)]
pub struct SubRegister {
    width: u8,
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Register {
    canonical_name: String,
    subregisters: Vec<SubRegister>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Machine {
    name: String,
    memory_widths: Vec<u8>,
    user_registers: Vec<Register>
}

impl Machine {
    pub fn new(file: &mut std::fs::File) -> Result<Machine, MachineError> {
        let mut file_contents: String = Default::default();
        let _ = file
            .read_to_string(&mut file_contents)
            .or(Err(MachineError::MachineFileParseError("".to_string())))?;
        TryInto::<Machine>::try_into(file_contents)
    }
}

impl TryFrom<String> for Machine {
    type Error = MachineError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&raw).map_err(|e| MachineError::MachineFileParseError(e.to_string()))
    }
}

