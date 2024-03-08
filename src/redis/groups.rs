use std::collections::HashSet;
use btcmbase::client::ClientID;

#[allow(unused_imports)]
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};

/// 异步函数，将指定用户添加到指定群组中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含要添加到群组的用户ID集合。
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
///     let users_to_add: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     add_group(&mut con, user_id, &users_to_add).await;
/// }
/// ```
pub async fn add_group(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_group_key(clt);
    
    // 使用cmd函数构建一个sadd命令，将HashSet中的数据写入Redis的set结构中
    let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，从指定群组中删除指定用户。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// - `hs`: HashSet<u64>，包含要从群组中删除的用户ID集合。
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
///     let users_to_remove: HashSet<u64> = [456, 789].iter().cloned().collect();
///     
///     del_group(&mut con, user_id, &users_to_remove).await;
/// }
/// ```
pub async fn del_group(con: &MultiplexedConnection, clt: ClientID, hs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_group_key(clt);
    
    // 使用cmd函数构建一个srem命令，将HashSet中的数据从Redis的set结构中删除
    let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取指定群组中的所有用户ID。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个包含群组中所有用户ID的HashSet<u64>。
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
///     let group_members = get_group(&mut con, user_id).await;
///     println!("Group members: {:?}", group_members);
/// }
/// ```
pub async fn get_group(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
    let mut con = con.clone();
    let key = get_group_key(clt);
    
    // 使用cmd函数构建一个smembers命令，获取Redis中的set结构中的数据，返回一个HashSet<u64>
    let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
    return result;
}

/// 异步函数，检查指定群组是否存在。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示群组是否存在。
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
///     let group_exists = exists_group(&mut con, user_id).await;
///     println!("Group exists: {:?}", group_exists);
/// }
/// ```
pub async fn exists_group(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let group_key = get_group_key(clt);
    
    // 调用redis-rs提供的exists方法，返回一个布尔值
    let result: bool = con.exists(group_key).await.unwrap();
    Ok(result)
}

/// 异步函数，从Redis中删除指定群组。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示群组是否成功删除。
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
///     let group_removed = remove_group(&mut con, user_id).await;
///     println!("Group removed: {:?}", group_removed);
/// }
/// ```
pub async fn remove_group(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let group_key = get_group_key(clt);
    
    // 调用redis-rs提供的del方法，返回一个布尔值表示是否成功删除
    let result: bool = con.del(group_key).await.unwrap();
    Ok(result)
}

/// 静态变量，表示群组键的前缀。
static GROUP_PREFIX: &str = "group:";

/// 获取群组键的函数。
/// 
/// # 参数
/// - `clt`: 指定的用户ID。
/// 
/// # 返回值
/// 返回一个字符串，表示与指定用户ID相关的群组键。
/// 
/// # 示例
/// ```rust
/// use btcmbase::client::ClientID;
/// 
/// let user_id = ClientID::from(123);
/// let group_key = get_group_key(user_id);
/// println!("Group key: {}", group_key);
/// ```
fn get_group_key(clt: ClientID) -> String {
    let group_id: u64 = clt.into();
    // 使用format!宏将两个变量连接成一个字符串变量
    format!("{}{}", GROUP_PREFIX, group_id)
}

// use std::collections::HashSet;
// use btcmbase::client::ClientID;


// #[allow(unused_imports)]
// use redis::{ aio::MultiplexedConnection, AsyncCommands, RedisResult };

// //
// pub async fn add_group(con: &MultiplexedConnection, clt:ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_group_key(clt);
//     let _: () = redis::cmd("SADD").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }
// //
// pub async fn del_group(con: &MultiplexedConnection, clt:ClientID, hs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_group_key(clt);
//     let _: () = redis::cmd("SREM").arg(key).arg(hs).query_async(&mut con).await.unwrap();
// }
// //
// pub async fn get_group(con: &MultiplexedConnection, clt:ClientID) -> HashSet<u64> {
//     let mut con = con.clone();
//     let key = get_group_key(clt);
//     // 使用cmd函数构建一个hgetall命令，获取redis中的hash结构中的数据，返回一个hashmap
//     let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
//     return result;
// }
// // 定义一个函数，用于检查指定的 key 是否存在
// pub async fn exists_group(con: &mut MultiplexedConnection, clt:ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let group_key = get_group_key(clt);
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.exists(group_key).await.unwrap();
//     Ok(result)
// }
// //
// pub async fn remove_group(con: &mut MultiplexedConnection, clt:ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let group_key = get_group_key(clt);
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.del(group_key).await.unwrap();
//     Ok(result)
// }

// static GROUP_PREFIX: &str = "group:";

// // key = users:clientid
// fn get_group_key(clt:ClientID) -> String {
//     let group_id :u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", GROUP_PREFIX, group_id)
// }

