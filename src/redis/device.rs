use std::collections::{HashMap, HashSet};
use btcmbase::client::ClientID;
use redis::{aio::MultiplexedConnection, AsyncCommands, RedisResult};

/// Redis中客户端设备相关键的前缀
static CLIENT_DEVICE_PREFIX: &str = "client_device:";

/// 根据客户端ID获取设备列表键的函数
fn get_clt_dev_list_key(clt: ClientID) -> String {
    let user_id: u64 = clt.into();
    format!("{}{}", CLIENT_DEVICE_PREFIX, user_id)
}

/// 异步函数，将设备ID集合添加到客户端的设备列表中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `devs`: HashSet<u64>，包含要添加的设备ID集合。
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
///     let client_id = ClientID::from(1001);
///     let devices: HashSet<u64> = [1, 2, 3].iter().cloned().collect();
///     
///     add_dev2clt(&mut con, client_id, &devices).await;
/// }
/// ```
pub async fn add_dev2clt(con: &MultiplexedConnection, clt: ClientID, devs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_clt_dev_list_key(clt);
    let _: () = redis::cmd("SADD").arg(key).arg(devs).query_async(&mut con).await.unwrap();
}

/// 异步函数，从客户端的设备列表中删除指定的设备ID集合。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `devs`: HashSet<u64>，包含要删除的设备ID集合。
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
///     let client_id = ClientID::from(1001);
///     let devices_to_remove: HashSet<u64> = [2, 3].iter().cloned().collect();
///     
///     del_dev4clt(&mut con, client_id, &devices_to_remove).await;
/// }
/// ```
pub async fn del_dev4clt(con: &MultiplexedConnection, clt: ClientID, devs: &HashSet<u64>) {
    let mut con = con.clone();
    let key = get_clt_dev_list_key(clt);
    let _: () = redis::cmd("SREM").arg(key).arg(devs).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取客户端的设备列表。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// 
/// # 返回值
/// 返回一个HashSet<u64>，包含客户端的所有设备ID。
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
///     let client_id = ClientID::from(1001);
///     
///     let dev_set = get_devclt_set(&mut con, client_id).await;
///     println!("Device set: {:?}", dev_set);
/// }
/// ```
pub async fn get_devclt_set(con: &MultiplexedConnection, clt: ClientID) -> HashSet<u64> {
    let mut con = con.clone();
    let key = get_clt_dev_list_key(clt);
    let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
    return result;
}

/// 异步函数，检查客户端的设备列表是否存在。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示设备列表是否存在。
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
///     let client_id = ClientID::from(1001);
///     
///     let exists = exists_devclt(&mut con, client_id).await.unwrap();
///     println!("Device list exists: {}", exists);
/// }
/// ```
pub async fn exists_devclt(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let key = get_clt_dev_list_key(clt);
    let result: bool = con.exists(key).await.unwrap();
    Ok(result)
}

/// 异步函数，删除客户端的设备列表。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示是否成功删除设备列表。
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
///     let client_id = ClientID::from(1001);
///     
///     let removed = remove_devclt_set(&mut con, client_id).await.unwrap();
///     println!("Device list removed: {}", removed);
/// }
/// ```
pub async fn remove_devclt_set(con: &mut MultiplexedConnection, clt: ClientID) -> RedisResult<bool> {
    let mut con = con.clone();
    let key = get_clt_dev_list_key(clt);
    let result: bool = con.del(key).await.unwrap();
    Ok(result)
}

/// 根据客户端ID和设备ID获取设备哈希键的函数
fn get_clt_dev_hash_key(clt: ClientID, dev: u32) -> String {
    let user_id: u64 = clt.into();
    format!("{}{}:{}", CLIENT_DEVICE_PREFIX, user_id, dev)
}

/// 异步函数，将设备信息哈希添加到客户端的设备哈希中。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `dev`: 设备ID。
/// - `hm`: HashMap<String, String>，包含设备信息的哈希映射。
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
///     let client_id = ClientID::from(1001);
///     let device_id = 1;
///     let device_info: HashMap<String, String> = [("name", "Device1"), ("type", "Smartphone")].iter().cloned().collect();
///     
///     add_dev2clt_hash(&mut con, client_id, device_id, &device_info).await;
/// }
/// ```
pub async fn add_dev2clt_hash(con: &MultiplexedConnection, clt: ClientID, dev: u32, hm: &HashMap<String, String>) {
    let mut con = con.clone();
    let user_key = get_clt_dev_hash_key(clt, dev);
    let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
}

/// 异步函数，获取客户端的指定设备信息。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `dev`: 设备ID。
/// 
/// # 返回值
/// 返回一个HashMap<String, String>，包含设备信息的哈希映射。
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
///     let client_id = ClientID::from(1001);
///     let device_id = 1;
///     
///     let device_info = get_device(&mut con, client_id, device_id).await;
///     println!("Device info: {:?}", device_info);
/// }
/// ```
pub async fn get_device(con: &MultiplexedConnection, clt: ClientID, dev: u32) -> HashMap<String, String> {
    let mut con = con.clone();
    let user_key = get_clt_dev_hash_key(clt, dev);
    let result: HashMap<String, String> = redis::cmd("HGETALL").arg(user_key).query_async(&mut con).await.unwrap();
    return result;
}

/// 异步函数，检查客户端的指定设备信息是否存在。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `dev`: 设备ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示设备信息是否存在。
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
///     let client_id = ClientID::from(1001);
///     let device_id = 1;
///     
///     let exists = exists_device(&mut con, client_id, device_id).await.unwrap();
///     println!("Device info exists: {}", exists);
/// }
/// ```
pub async fn exists_device(con: &mut MultiplexedConnection, clt: ClientID, dev: u32) -> RedisResult<bool> {
    let mut con = con.clone();
    let user_key = get_clt_dev_hash_key(clt, dev);
    let result: bool = con.exists(user_key).await.unwrap();
    Ok(result)
}

/// 异步函数，删除客户端的指定设备信息。
/// 
/// # 参数
/// - `con`: Redis的MultiplexedConnection，用于与Redis进行异步通信。
/// - `clt`: 客户端ID。
/// - `dev`: 设备ID。
/// 
/// # 返回值
/// 返回一个RedisResult<bool>，表示是否成功删除设备信息。
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
///     let client_id = ClientID::from(1001);
///     let device_id = 1;
///     
///     let removed = remove_device(&mut con, client_id, device_id).await.unwrap();
///     println!("Device removed: {}", removed);
/// }
/// ```
pub async fn remove_device(con: &mut MultiplexedConnection, clt: ClientID, dev: u32) -> RedisResult<bool> {
    let mut con = con.clone();
    let user_key = get_clt_dev_hash_key(clt, dev);
    let result: bool = con.del(user_key).await.unwrap();
    Ok(result)
}

// use std::collections::{HashMap, HashSet};
// use btcmbase::client::ClientID;
// use redis::{ aio::MultiplexedConnection, AsyncCommands, RedisResult };

// // client_device:1001      -> HashSet
// // client_device:1001:0001 -> Hash
// static CLIENT_DEVICE_PREFIX:  &str = "client_device:";


// fn get_clt_dev_list_key(clt:ClientID) -> String {
//     let user_id :u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}", CLIENT_DEVICE_PREFIX, user_id)
// }
// //
// pub async fn add_dev2clt(con: &MultiplexedConnection, clt:ClientID, devs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_clt_dev_list_key(clt);
//     let _: () = redis::cmd("SADD").arg(key).arg(devs).query_async(&mut con).await.unwrap();
// }
// //
// pub async fn del_dev4clt(con: &MultiplexedConnection, clt:ClientID, devs: &HashSet<u64>) {
//     let mut con = con.clone();
//     let key = get_clt_dev_list_key(clt);
//     let _: () = redis::cmd("SREM").arg(key).arg(devs).query_async(&mut con).await.unwrap();
// }
// //
// pub async fn get_devclt_set(con: &MultiplexedConnection, clt:ClientID) -> HashSet<u64> {
//     let mut con = con.clone();
//     let key = get_clt_dev_list_key(clt);
//     // 使用cmd函数构建一个hgetall命令，获取redis中的hash结构中的数据，返回一个hashmap
//     let result: HashSet<u64> = redis::cmd("SMEMBERS").arg(key).query_async(&mut con).await.unwrap();
//     return result;
// }
// // 定义一个函数，用于检查指定的 key 是否存在
// pub async fn exists_devclt(con: &mut MultiplexedConnection, clt:ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let group_key = get_clt_dev_list_key(clt);
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.exists(group_key).await.unwrap();
//     Ok(result)
// }
// //
// pub async fn remove_devclt_set(con: &mut MultiplexedConnection, clt:ClientID) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let group_key = get_clt_dev_list_key(clt);
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.del(group_key).await.unwrap();
//     Ok(result)
// }
// //
// fn get_clt_dev_hash_key(clt:ClientID,dev:u32) -> String {
//     let user_id :u64 = clt.into();
//     // 使用format!宏将两个变量连接成一个字符串变量
//     format!("{}{}:{}", CLIENT_DEVICE_PREFIX, user_id,dev)
// }
// //
// pub async fn add_dev2clt_hash(con: &MultiplexedConnection, clt:ClientID, dev:u32, hm: &HashMap<String, String>) {
//     let mut con = con.clone();
//     let user_key = get_clt_dev_hash_key(clt,dev);
//     // 使用cmd函数构建一个hset命令，将hashmap中的数据写入redis的hash结构中，假设hash的名字是"user"
//     // let _: () = con.hset_multiple(user_key, hm).await.unwrap();
//     let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
// }
// //
// pub async fn get_device(con: &MultiplexedConnection, clt:ClientID,dev:u32) -> HashMap<String, String> {
//     let mut con = con.clone();
//     let user_key = get_clt_dev_hash_key(clt,dev);
//     // 使用cmd函数构建一个hgetall命令，获取redis中的hash结构中的数据，返回一个hashmap
//     let result: HashMap<String, String> = redis
//         ::cmd("HGETALL")
//         .arg(user_key)
//         .query_async(&mut con).await
//         .unwrap();
//     return result;
// }
// // 
// pub async fn exists_device(con: &mut MultiplexedConnection, clt:ClientID,dev:u32) -> RedisResult<bool> {
//     let mut con = con.clone();
//     let user_key = get_clt_dev_hash_key(clt,dev);
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.exists(user_key).await.unwrap();
//     Ok(result)
// }
// // 
// pub async fn remove_device(con: &mut MultiplexedConnection, clt:ClientID,dev:u32) -> RedisResult<bool>{
//     // 先获取用户的基本信息,用于存储到删除用户列表中
//     // let hm = get_user(con,clt).await;
//     let mut con = con.clone();
//     let user_key = get_clt_dev_hash_key(clt,dev);
//     //let result : bool = 
//     // con.rename(user_key, del_user_key).await
//     // 调用 redis-rs 提供的 exists 方法，返回一个布尔值
//     let result: bool = con.del(user_key).await.unwrap();
//     // 将已删除的用户信息
//     // let user_key = get_del_user_key(clt);
//     // let _: () = redis::cmd("HSET").arg(user_key).arg(hm).query_async(&mut con).await.unwrap();
//     Ok(result)
// }