use std::str::FromStr;

pub enum StringCommandType {
    SET,
    GET,
    INCR,
    DECR,
    INCRBY,
    DECRBY,
    MGET,
    MSET,
}

pub enum ListCommandType {
    LPUSH,
    LPOP,
    LRANGE,
    LLEN,
}

pub enum CommandType {
    PING,
    CONFIG,
    STRING(StringCommandType),
    LIST(ListCommandType),
}

const STRING_COMMANDS: &[&str] = &[
    "SET", "GET", "INCR", "DECR", "INCRBY", "DECRBY", "MGET", "MSET",
];
const LIST_COMMANDS: &[&str] = &["LPUSH", "LPOP", "LRANGE", "LLEN"];

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::PING),
            "CONFIG" => Ok(CommandType::CONFIG),
            s if STRING_COMMANDS.contains(&s) => {
                Ok(CommandType::STRING(StringCommandType::from_str(s)?))
            }
            s if LIST_COMMANDS.contains(&s) => Ok(CommandType::LIST(ListCommandType::from_str(s)?)),
            _ => Err(()),
        }
    }
}

impl FromStr for StringCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<StringCommandType, Self::Err> {
        match s {
            "SET" => Ok(StringCommandType::SET),
            "GET" => Ok(StringCommandType::GET),
            "INCR" => Ok(StringCommandType::INCR),
            "DECR" => Ok(StringCommandType::DECR),
            "INCRBY" => Ok(StringCommandType::INCRBY),
            "DECRBY" => Ok(StringCommandType::DECRBY),
            "MGET" => Ok(StringCommandType::MGET),
            "MSET" => Ok(StringCommandType::MSET),
            _ => Err(()),
        }
    }
}

impl FromStr for ListCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<ListCommandType, Self::Err> {
        match s {
            "LPUSH" => Ok(ListCommandType::LPUSH),
            "LPOP" => Ok(ListCommandType::LPOP),
            "LRANGE" => Ok(ListCommandType::LRANGE),
            "LLEN" => Ok(ListCommandType::LLEN),
            _ => Err(()),
        }
    }
}
