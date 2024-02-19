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

pub enum SetCommandType {
    Add,
    Rem,
    Members,
    IsMember,
    Card,
}

pub enum CommandType {
    Ping,
    String(StringCommandType),
    List(ListCommandType),
    Set(SetCommandType),
}

const STRING_COMMANDS: &[&str] = &[
    "set", "get", "incr", "decr", "incrby", "decrby", "mget", "mset",
];
const LIST_COMMANDS: &[&str] = &["lpush", "lpop", "lrange", "llen", "rpush", "rpop"];
const SET_COMMANDS: &[&str] = &["sadd", "srem", "smembers", "sismember", "scard"];

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "ping" => Ok(CommandType::Ping),
            s if STRING_COMMANDS.contains(&s) => {
                Ok(CommandType::String(StringCommandType::from_str(s)?))
            }
            s if LIST_COMMANDS.contains(&s) => Ok(CommandType::List(ListCommandType::from_str(s)?)),
            s if SET_COMMANDS.contains(&s) => Ok(CommandType::Set(SetCommandType::from_str(s)?)),
            _ => Err(()),
        }
    }
}

impl FromStr for StringCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<StringCommandType, Self::Err> {
        match s {
            "set" => Ok(StringCommandType::Set),
            "get" => Ok(StringCommandType::Get),
            "incr" => Ok(StringCommandType::Incr),
            "decr" => Ok(StringCommandType::Decr),
            "incrby" => Ok(StringCommandType::IncrBy),
            "decrby" => Ok(StringCommandType::DecrBy),
            "mget" => Ok(StringCommandType::MGet),
            "mset" => Ok(StringCommandType::MSet),
            _ => Err(()),
        }
    }
}

impl FromStr for ListCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<ListCommandType, Self::Err> {
        match s {
            "lpush" => Ok(ListCommandType::LPush),
            "lpop" => Ok(ListCommandType::LPop),
            "lrange" => Ok(ListCommandType::LRange),
            "llen" => Ok(ListCommandType::LLen),
            "rpush" => Ok(ListCommandType::RPush),
            "rpop" => Ok(ListCommandType::RPop),
            _ => Err(()),
        }
    }
}

impl FromStr for SetCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<SetCommandType, Self::Err> {
        match s {
            "sadd" => Ok(SetCommandType::Add),
            "srem" => Ok(SetCommandType::Rem),
            "smembers" => Ok(SetCommandType::Members),
            "sismember" => Ok(SetCommandType::IsMember),
            "scard" => Ok(Self::Card),
            _ => Err(()),
        }
    }
}
