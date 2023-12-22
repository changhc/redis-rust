use std::str::FromStr;

pub enum CommandType {
    PING,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::PING),
            _ => Err(()),
        }
    }
}
