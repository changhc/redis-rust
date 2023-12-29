use std::str::FromStr;

pub enum CommandType {
    PING,
    SET,
    GET,
    INCR,
    DECR,
    INCRBY,
    DECRBY,
    LPUSH,
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
            "DECRBY" => Ok(CommandType::DECRBY),
            "LPUSH" => Ok(CommandType::LPUSH),
            _ => Err(()),
        }
    }
}
