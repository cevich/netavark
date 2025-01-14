use serde;
use serde_json;

use std::{fmt, io, error::Error, fs::File};

#[derive(Debug)]
pub enum SerializeError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SerializeError::Io(ref err) => err.fmt(f),
            SerializeError::Json(ref err) => err.fmt(f),
        }
    }
}

#[allow(deprecated)]
impl Error for SerializeError {
    fn description(&self) -> &str {
        match *self {
            SerializeError::Io(ref err) => err.description(),
            SerializeError::Json(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SerializeError::Io(ref err) => Some(err),
            SerializeError::Json(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for SerializeError {
    fn from(err: io::Error) -> SerializeError {
        SerializeError::Io(err)
    }
}

impl From<serde_json::Error> for SerializeError {
    fn from(err: serde_json::Error) -> SerializeError {
        SerializeError::Json(err)
    }
}

pub fn serialize<T: serde::Serialize>(obj: &T, path: &str) -> Result<(), SerializeError> {
    let mut file = File::create(path)?;
    Ok(serde_json::to_writer_pretty(&mut file, &obj)?)
}

pub fn deserialize<T>(path: &str) -> Result<T, SerializeError>
    where for<'de> T: serde::Deserialize<'de>
{
    let file = File::open(path)?;
    Ok(serde_json::from_reader(&file)?)
}

pub fn to_writer<W: io::Write, T: serde::Serialize>(obj: &T, mut writer: W) -> Result<(), SerializeError> {
    Ok(serde_json::to_writer(&mut writer, &obj)?)
}

pub fn to_string<T: serde::Serialize>(obj: &T) -> Result<String, SerializeError> {
    Ok(serde_json::to_string(&obj)?)
}

