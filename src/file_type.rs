use std::{str::FromStr, fmt::Display, io::{Write, Read}, fs::File};


#[derive(Clone)]
pub (crate) enum FileType {
    File(String),
    Stream
}

impl FileType {
    pub fn as_writer(&self) -> std::io::Result<Box<dyn Write>> {
        match self {
            Self::File(s) => match File::create(s) {
                Ok(w) => Ok(Box::new(w) as Box<dyn Write>),
                Err(why) => Err(why)
            }
            Self::Stream => Ok(Box::new(std::io::stdout()) as Box<dyn Write>)
        }
    }

    pub fn as_reader(&self) -> std::io::Result<Box<dyn Read>> {
        match self {
            Self::File(s) => match File::open(s) {
                Ok(w) => Ok(Box::new(w) as Box<dyn Read>),
                Err(why) => Err(why)
            }
            Self::Stream => Ok(Box::new(std::io::stdin()) as Box<dyn Read>)
        }
    }
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            if s == "-" { Self::Stream }
            else        { Self::File(s.to_owned()) }
        )
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::File(s) => write!(f, "{}", s),
            FileType::Stream => write!(f, "-"),
        }
    }
}