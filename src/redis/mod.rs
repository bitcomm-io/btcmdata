pub mod users;
pub mod groups;
pub mod inbox;
pub mod outbox;
pub mod device;


use std::sync::Arc;
use redis::RedisError;
use redis::aio::MultiplexedConnection;
use once_cell::sync::OnceCell;

/// # Redis Database Manager
///
/// The `RedisDBManager` struct represents a manager for interacting with a Redis database.
/// It includes a Redis client and a multiplexed connection to the database.
///
/// # Examples
///
/// ```rust
/// use redis_db::init_redis_database;
///
/// #[tokio::main]
/// async fn main() {
///     // Initialize the Redis database manager
///     let redis_url = "redis://127.0.0.1/";
///     let manager = init_redis_database(redis_url).await.expect("Failed to initialize Redis database.");
///
///     // Access the Redis client and connection
///     let client = manager.client;
///     let connection = manager.connect;
///
///     // Perform database operations...
/// }
/// ```
#[derive(Debug)]
// 
pub struct RedisDBManager {
    // #[getset(get = "pub")]
    client: redis::Client,
    // #[getset(get = "pub")]
    connect: MultiplexedConnection,
}

// 使用 OnceCell 包装 Singleton，确保只初始化一次
static SINGLETON_REDIS_DB_MANAGER: OnceCell<Arc<RedisDBManager>> = OnceCell::new();

/// Initializes the Redis database and returns a Result containing an Arc-wrapped `RedisDBManager`.
///
/// # Arguments
///
/// * `redis_url` - A string representing the Redis server URL.
///
/// # Returns
///
/// Returns a `Result` containing an `Arc<RedisDBManager>` on success, or a `RedisError` on failure.
///
/// # Examples
///
/// ```rust
/// use redis_db::init_redis_database;
///
/// #[tokio::main]
/// async fn main() {
///     let redis_url = "redis://127.0.0.1/";
///     let manager = init_redis_database(redis_url).await.expect("Failed to initialize Redis database.");
///     // Use the manager for database operations...
/// }
/// ```
pub async fn init_redis_database(redis_url: &str) -> Result<Arc<RedisDBManager>, RedisError> {
    let clt = redis::Client::open(redis_url)?;
    let con = clt.get_multiplexed_tokio_connection().await?;
    
    // Initialize the singleton instance
    SINGLETON_REDIS_DB_MANAGER.get_or_init(|| {
        Arc::new(RedisDBManager {
            client: clt.clone(),
            connect: con.clone(),
        })
    });

    Ok(SINGLETON_REDIS_DB_MANAGER.get().cloned().unwrap())
}

/// Gets the Redis database manager from the singleton instance.
///
/// # Returns
///
/// Returns an `Option<Arc<RedisDBManager>>`. If the singleton instance exists, it returns the manager;
/// otherwise, it returns `None`.
///
/// # Examples
///
/// ```rust
/// use redis_db::get_redis_dbmanager;
///
/// #[tokio::main]
/// async fn main() {
///     if let Some(manager) = get_redis_dbmanager() {
///         // Use the manager for database operations...
///     } else {
///         println!("RedisDBManager singleton instance does not exist.");
///     }
/// }
/// ```
pub fn get_redis_dbmanager() -> Option<Arc<RedisDBManager>> {
    SINGLETON_REDIS_DB_MANAGER.get().cloned()
}

/// Gets the Redis client from the singleton instance.
///
/// # Returns
///
/// Returns an `Option<redis::Client>`. If the singleton instance exists, it returns the client;
/// otherwise, it returns `None`.
///
/// # Examples
///
/// ```rust
/// use redis_db::get_redis_client;
///
/// #[tokio::main]
/// async fn main() {
///     if let Some(client) = get_redis_client() {
///         // Use the client for direct Redis interactions...
///     } else {
///         println!("Redis client singleton instance does not exist.");
///     }
/// }
/// ```
pub fn get_redis_client() -> Option<redis::Client> {
    SINGLETON_REDIS_DB_MANAGER.get().map(|manager| manager.client.clone())
}

/// Gets the Redis multiplexed connection from the singleton instance.
///
/// # Returns
///
/// Returns an `Option<MultiplexedConnection>`. If the singleton instance exists, it returns the connection;
/// otherwise, it returns `None`.
///
/// # Examples
///
/// ```rust
/// use redis_db::get_redis_connect;
///
/// #[tokio::main]
/// async fn main() {
///     if let Some(connection) = get_redis_connect() {
///         // Use the connection for direct Redis interactions...
///     } else {
///         println!("Redis connection singleton instance does not exist.");
///     }
/// }
/// ```
pub fn get_redis_connect() -> Option<MultiplexedConnection> {
    SINGLETON_REDIS_DB_MANAGER.get().map(|manager| manager.connect.clone())
}
