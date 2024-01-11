use std::str::FromStr;

pub enum StringCommandType {
    Set,
    Get,
    Incr,
    Decr,
    IncrBy,
    DecrBy,
    MGet,
    MSet,
}

pub enum ListCommandType {
    LPush,
    LPop,
    LRange,
    LLen,
    RPush,
    RPop,
}

pub enum CommandType {
    Ping,
    String(StringCommandType),
    List(ListCommandType),
}

const STRING_COMMANDS: &[&str] = &[
    "SET", "GET", "INCR", "DECR", "INCRBY", "DECRBY", "MGET", "MSET",
];
const LIST_COMMANDS: &[&str] = &["LPUSH", "LPOP", "LRANGE", "LLEN", "RPUSH", "RPOP"];

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::Ping),
            s if STRING_COMMANDS.contains(&s) => {
                Ok(CommandType::String(StringCommandType::from_str(s)?))
            }
            s if LIST_COMMANDS.contains(&s) => Ok(CommandType::List(ListCommandType::from_str(s)?)),
            _ => Err(()),
        }
    }
}

impl FromStr for StringCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<StringCommandType, Self::Err> {
        match s {
            "SET" => Ok(StringCommandType::Set),
            "GET" => Ok(StringCommandType::Get),
            "INCR" => Ok(StringCommandType::Incr),
            "DECR" => Ok(StringCommandType::Decr),
            "INCRBY" => Ok(StringCommandType::IncrBy),
            "DECRBY" => Ok(StringCommandType::DecrBy),
            "MGET" => Ok(StringCommandType::MGet),
            "MSET" => Ok(StringCommandType::MSet),
            _ => Err(()),
        }
    }
}

impl FromStr for ListCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<ListCommandType, Self::Err> {
        match s {
            "LPUSH" => Ok(ListCommandType::LPush),
            "LPOP" => Ok(ListCommandType::LPop),
            "LRANGE" => Ok(ListCommandType::LRange),
            "LLEN" => Ok(ListCommandType::LLen),
            "RPUSH" => Ok(ListCommandType::RPush),
            "RPOP" => Ok(ListCommandType::RPop),
            _ => Err(()),
        }
    }
}
