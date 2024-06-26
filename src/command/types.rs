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
    Diff,
}

pub enum HashCommandType {
    Set,
    Get,
    GetAll,
    IncrBy,
}

pub enum SortedSetCommandType {
    Add,
    Range,
    Rem,
    Rank,
}

pub enum StreamCommandType {
    Add,
}

pub enum CommandType {
    Ping,
    String(StringCommandType),
    List(ListCommandType),
    Set(SetCommandType),
    Hash(HashCommandType),
    SortedSet(SortedSetCommandType),
    Stream(StreamCommandType),
}

const STRING_COMMANDS: &[&str] = &[
    "set", "get", "incr", "decr", "incrby", "decrby", "mget", "mset",
];
const LIST_COMMANDS: &[&str] = &["lpush", "lpop", "lrange", "llen", "rpush", "rpop"];
const SET_COMMANDS: &[&str] = &["sadd", "srem", "smembers", "sismember", "scard", "sdiff"];
const HASH_COMMANDS: &[&str] = &["hset", "hget", "hgetall", "hincrby"];
const SORTED_SET_COMMANDS: &[&str] = &["zadd", "zrange", "zrem", "zrank"];
const STREAM_COMMANDS: &[&str] = &["xadd"];

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
            s if HASH_COMMANDS.contains(&s) => Ok(CommandType::Hash(HashCommandType::from_str(s)?)),
            s if SORTED_SET_COMMANDS.contains(&s) => {
                Ok(CommandType::SortedSet(SortedSetCommandType::from_str(s)?))
            }
            s if STREAM_COMMANDS.contains(&s) => {
                Ok(CommandType::Stream(StreamCommandType::from_str(s)?))
            }
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
            "scard" => Ok(SetCommandType::Card),
            "sdiff" => Ok(SetCommandType::Diff),
            _ => Err(()),
        }
    }
}

impl FromStr for HashCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<HashCommandType, Self::Err> {
        match s {
            "hset" => Ok(HashCommandType::Set),
            "hget" => Ok(HashCommandType::Get),
            "hgetall" => Ok(HashCommandType::GetAll),
            "hincrby" => Ok(HashCommandType::IncrBy),
            _ => Err(()),
        }
    }
}

impl FromStr for SortedSetCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<SortedSetCommandType, Self::Err> {
        match s {
            "zadd" => Ok(SortedSetCommandType::Add),
            "zrange" => Ok(SortedSetCommandType::Range),
            "zrem" => Ok(SortedSetCommandType::Rem),
            "zrank" => Ok(SortedSetCommandType::Rank),
            _ => Err(()),
        }
    }
}

impl FromStr for StreamCommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<StreamCommandType, Self::Err> {
        match s {
            "xadd" => Ok(StreamCommandType::Add),
            _ => Err(()),
        }
    }
}
