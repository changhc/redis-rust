use std::str::FromStr;

pub enum CommandType {
    PING,
    SET,
    GET,
    INCR,
    DECR,
    INCRBY,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::PING),
            "SET" => Ok(CommandType::SET),
            "GET" => Ok(CommandType::GET),
            "INCR" => Ok(CommandType::INCR),
            "DECR" => Ok(CommandType::DECR),
            "INCRBY" => Ok(CommandType::INCRBY),
            _ => Err(()),
        }
    }
}
