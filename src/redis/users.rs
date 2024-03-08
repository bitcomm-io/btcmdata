use std::collections::{HashMap, HashSet};
use btcmbase::client::ClientID;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};

/// 异步函数，将用户信息添加到Redis中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hm`: HashMap<String, String>，包含用户信息的哈希表。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashMap;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     let user_info: HashMap<String, String> = [("name", "John"), ("age", "30")].iter().cloned().collect();
///     
///     add_user(&mut con, user_id, &user_info).await;
/// }
/// ```
pub async fn add_user(con: &MultiplexedConnection, clt: ClientID, hm: &HashMap<String, String>) {
    let mut con = con.clone();
    let user_key = get_user_key(clt);
    
    // 使用cmd函数构建一个hset命令，将HashMap中的数据写入Redis的hash结构中，假设hash的名字是"user"
    let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取指定用户的信息。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个HashMap<String, String>，包含用户的信息。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashMap;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     
///     let user_info = get_user(&mut con, user_id).await;
///     println!("User info: {:?}", user_info);
/// }
/// ```
pub async fn get_user(con: &MultiplexedConnection, clt: ClientID) -> HashMap<String, String> {
    let mut con = con.clone();
    let user_key = get_user_key(clt);
    
    // 使用cmd函数构建一个hgetall命令，获取Redis中的hash结构中的数据，返回一个HashMap
    let result: HashMap<String, String> = redis
        ::cmd("HGETALL")
        .arg(user_key)
        .query_async(&mut con)
        .await
        .unwrap();
    
    return result;
}

/// 异步函数，检查指定用户是否存在。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示用户是否存在。
/// 
/// # 示例
/// ```rust
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     
///     let user_exists = exists_user(&mut con, user_id).await;
///     println!("User exists: {:?}", user_exists);
/// }
/// ```
pub async fn exists_user(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let user_key = get_user_key(clt);
    
    // 调用redis-rs提供的exists方法，返回一个布尔值
    let result: bool = con.exists(user_key).await.unwrap();
    Ok(result)
}

/// 异步函数，从Redis中删除指定用户。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示用户是否成功删除。
/// 
/// # 示例
/// ```rust
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     
///     let user_removed = remove_user(&mut con, user_id).await;
///     println!("User removed: {:?}", user_removed);
/// }
/// ```
pub async fn remove_user(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let user_key = get_user_key(clt);
    let del_user_key = get_del_user_key(clt);
    
    // 将用户键重命名为删除用户键
    con.rename(user_key, del_user_key).await
    
    // 这里注释了原来的删除用户的代码，因为在rename时已经将用户移动到了删除用户键
    // let result: bool = con.del(user_key).await.unwrap();
    
    // 将已删除的用户信息存储到删除用户键中（未注释的代码）
    // let user_key = get_del_user_key(clt);
    // let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
    // Ok(result)
}

/// 用户键的前缀
static USER_PREFIX: &str = "users:";

/// 获取用户键的函数
fn get_user_key(clt: ClientID) -> String {
    let user_id: u64 = clt.into();
    // 使用format!宏将两个变量连接成一个字符串变量
    format!("{}{}", USER_PREFIX, user_id)
}

/// 删除用户键的前缀
static DEL_USER_PREFIX: &str = "del_users:";

/// 获取删除用户键的函数
fn get_del_user_key(clt: ClientID) -> String {
    let user_id: u64 = clt.into();
    // 使用format!宏将两个变量连接成一个字符串变量
    format!("{}{}", DEL_USER_PREFIX, user_id)
}

/// 用户联系人键的前缀
static USER_CONTS_PREFIX: &str = "conts_user:";

/// 获取用户联系人键的函数
fn get_user_conts_key(clt: ClientID) -> String {
    let user_id: u64 = clt.into();
    // 使用format!宏将两个变量连接成一个字符串变量
    format!("{}{}", USER_CONTS_PREFIX, user_id)
}

/// 异步函数，将用户联系人添加到Redis中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含用户联系人的用户ID集合。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     let contacts: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     add_user_contacts(&mut con, user_id, &contacts).await;
/// }
/// ```
pub async fn add_user_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_user_conts_key(clt);
    
    // 使用cmd函数构建一个sadd命令，将HashSet中的数据写入Redis的set结构中
    let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，从Redis中删除指定用户的联系人。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含要删除的用户联系人的用户ID集合。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     let contacts_to_remove: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     del_user_contacts(&mut con, user_id, &contacts_to_remove).await;
/// }
/// ```
pub async fn del_user_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_user_conts_key(clt);
    
    // 使用cmd函数构建一个srem命令，将HashSet中的数据从Redis的set结构中删除
    let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取指定用户的所有联系人。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个HashSet<u64>，包含用户的所有联系人的用户ID。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     
///     let user_contacts = get_user_contacts(&mut con, user_id).await;
///     println!("User contacts: {:?}", user_contacts);
/// }
/// ```
pub async fn get_user_contacts(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
    let mut con = con.clone();
    let key = get_user_conts_key(clt);
    
    // 使用cmd函数构建一个smembers命令，获取Redis中的set结构中的数据，返回一个HashSet<u64>
    let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
    return result;
}

/// 组联系人键的前缀
static GROUP_CONTS_PREFIX: &str = "conts_group:";

/// 获取组联系人键的函数
fn get_group_conts_key(clt: ClientID) -> String {
    let user_id: u64 = clt.into();
    // 使用format!宏将两个变量连接成一个字符串变量
    format!("{}{}", GROUP_CONTS_PREFIX, user_id)
}

/// 异步函数，将组联系人添加到Redis中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含组联系人的用户ID集合。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     let group_contacts: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     add_group_contacts(&mut con, user_id, &group_contacts).await;
/// }
/// ```
pub async fn add_group_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_group_conts_key(clt);
    
    // 使用cmd函数构建一个sadd命令，将HashSet中的数据写入Redis的set结构中
    let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，从Redis中删除指定用户组的联系人。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含要删除的用户组联系人的用户ID集合。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     let group_contacts_to_remove: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     del_group_contacts(&mut con, user_id, &group_contacts_to_remove).await;
/// }
/// ```
pub async fn del_group_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_group_conts_key(clt);
    
    // 使用cmd函数构建一个srem命令，将HashSet中的数据从Redis的set结构中删除
    let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取指定用户组的所有联系人。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个HashSet<u64>，包含用户组的所有联系人的用户ID。
/// 
/// # 示例
/// ```rust
/// use std::collections::HashSet;
/// use btcmbase::client::ClientID;
/// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
///     let mut con = client.get_async_connection().await.expect("Failed to get Redis connection");
///     let user_id = ClientID::from(123);
///     
///     let group_contacts = get_group_contacts(&mut con, user_id).await;
///     println!("Group contacts: {:?}", group_contacts);
/// }
/// ```
pub async fn get_group_contacts(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
    let mut con = con.clone();
    let key = get_group_conts_key(clt);
    
    // 使用cmd函数构建一个smembers命令，获取Redis中的set结构中的数据，返回一个HashSet<u64>
    let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
    return result;
}


// use std::collections::{HashMap, HashSet};
// use btcmbase::client::ClientID;
// use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};

// // 添加用户到Redis中的异步函数
// pub async fn add_user(con: &MultiplexedConnection, clt: ClientID, hm: &HashMap<String, String>) {
//     // 克隆连接，确保异步函数中不会修改原始连接
//     let mut con = con.clone();
//     // 获取用户在Redis中的键
//     let user_key = get_user_key(clt);
    
//     // 使用cmd函数构建一个hset命令，将hashmap中的数据写入redis的hash结构中，假设hash的名字是"user"
//     let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
// }

// // 获取用户信息的异步函数
// pub async fn get_user(con: &MultiplexedConnection, clt: ClientID) -> HashMap<String, String> {
//     let mut con = con.clone();
//     let user_key = get_user_key(clt);
    
//     // 使用cmd函数构建一个hgetall命令，获取redis中的hash结构中的数据，返回一个hashmap
//     let result: HashMap<String, String> = redis
//         ::cmd("HGETALL")
//         .arg(user_key)
//         .query_async(&mut con)
//         .await
//         .unwrap();
    
//     return result;
// }

// // 检查用户是否存在的异步函数
// pub async fn exists_user(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let user_key = get_user_key(clt);
    
//     // 调用redis-rs提供的exists方法，返回一个布尔值
//     let result: bool = con.exists(user_key).await.unwrap();
//     Ok(result)
// }

// // 删除用户的异步函数
// pub async fn remove_user(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let user_key = get_user_key(clt);
//     let del_user_key = get_del_user_key(clt);
    
//     // 将用户键重命名为删除用户键
//     con.rename(user_key, del_user_key).await
    
//     // 这里注释了原来的删除用户的代码，因为在rename时已经将用户移动到了删除用户键
//     // let result: bool = con.del(user_key).await.unwrap();
    
//     // 将已删除的用户信息存储到删除用户键中（未注释的代码）
//     // let user_key = get_del_user_key(clt);
//     // let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
//     // Ok(result)
// }

// // 用户键的前缀
// static USER_PREFIX: &str = "users:";

// // 获取用户键的函数
// fn get_user_key(clt: ClientID) -> String {
//     let user_id: u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", USER_PREFIX, user_id)
// }

// // 删除用户键的前缀
// static DEL_USER_PREFIX: &str = "del_users:";

// // 获取删除用户键的函数
// fn get_del_user_key(clt: ClientID) -> String {
//     let user_id: u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", DEL_USER_PREFIX, user_id)
// }

// // 用户联系人键的前缀
// static USER_CONTS_PREFIX: &str = "conts_user:";

// // 获取用户联系人键的函数
// fn get_user_conts_key(clt: ClientID) -> String {
//     let user_id: u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", USER_CONTS_PREFIX, user_id)
// }

// // 添加用户联系人到Redis中的异步函数
// pub async fn add_user_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_user_conts_key(clt);
    
//     // 使用cmd函数构建一个sadd命令，将hashset中的数据写入redis的set结构中
//     let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }

// // 删除用户联系人的异步函数
// pub async fn del_user_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_user_conts_key(clt);
    
//     // 使用cmd函数构建一个srem命令，将hashset中的数据从redis的set结构中删除
//     let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }

// // 获取用户联系人的异步函数
// pub async fn get_user_contacts(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
//     let mut con = con.clone();
//     let key = get_user_conts_key(clt);
    
//     // 使用cmd函数构建一个smembers命令，获取redis中的set结构中的数据，返回一个hashset
//     let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
//     return result;
// }

// // 组联系人键的前缀
// static GROUP_CONTS_PREFIX: &str = "conts_group:";

// // 获取组联系人键的函数
// fn get_group_conts_key(clt: ClientID) -> String {
//     let user_id: u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", GROUP_CONTS_PREFIX, user_id)
// }

// // 添加组联系人到Redis中的异步函数
// pub async fn add_group_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_group_conts_key(clt);
    
//     // 使用cmd函数构建一个sadd命令，将hashset中的数据写入redis的set结构中
//     let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }
// // 删除组联系人的异步函数
// pub async fn del_group_contacts(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_group_conts_key(clt);
    
//     // 使用cmd函数构建一个srem命令，将hashset中的数据从redis的set结构中删除
//     let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }

// // 获取组联系人的异步函数
// pub async fn get_group_contacts(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
//     let mut con = con.clone();
//     let key = get_group_conts_key(clt);
    
//     // 使用cmd函数构建一个smembers命令，获取redis中的set结构中的数据，返回一个hashset
//     let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
//     return result;
// }