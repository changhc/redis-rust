use std::str::FromStr;

pub enum CommandType {
    PING,
    SET,
    GET,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::PING),
            "SET" => Ok(CommandType::SET),
            "GET" => Ok(CommandType::GET),
            _ => Err(()),
        }
    }
}
