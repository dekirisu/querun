#![allow(dead_code)]
use derive_error::Error;
use redis::{RedisError, aio};
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use serde_redis::RedisDeserialize;

pub use querun_derive::*;
pub use querio::*;
pub use querio_redisgraph::*;

pub trait QuerunOutput: Clone + DeserializeOwned {}

/// Types of errors, the query could encounter.
#[derive(Debug,Error)]
pub enum QueryError {
    /// The query has no return at all, this is an error due to simplification of the crate.
    /// 
    /// ❗ This might get deleted in future versions.
    NoReturn,
    /// Since the crate generates a redis graph query return as json type, the return has to be a string!
    /// 
    /// ❗ This might get deleted in future versions.
    NoStringReturn,
    /// The more low-level result structure isn't right, so that the result in itself can't be right.
    /// This one might never occure in production code. 🎉
    NoResult,
    /// Something about the query you've entered is wrong.
    Query
}

pub struct QuerunRedisGraphExe;
impl QuerunRedisGraphExe {
    /// Executes the query on the Redis-Server handed over by the [`redis::aio::ConnectionManager`]
    /// 
    /// [`redis::aio::ConnectionManager`]:https://docs.rs/redis/0.21.5/redis/aio/struct.ConnectionManager.html
    async fn result_async<C>(con: &mut C, query: String) -> Result<String,QueryError>
        where C: aio::ConnectionLike
    {
        let res: Result<Vec<redis::Value>, RedisError> = 
            redis::cmd("GRAPH.QUERY").arg("Main").arg(query).query_async(con).await;
        let out = if let Ok(res) = res {
            if res.len()==3 {
                let b: Result<Vec<Vec<String>>, serde_redis::decode::Error> = res[1].deserialize();
                if let Ok(out) = b {
                    if out.len()==1 {
                        Ok(out[0][0].to_owned())
                    } else { Err(QueryError::NoReturn) }
                } else { Err(QueryError::NoStringReturn) }
            } else { Err(QueryError::NoResult) }
        } else { Err(QueryError::Query) };
        out
    }
    async fn execute_async<C>(con: &mut C, query: String) -> Result<(),QueryError> 
        where C: aio::ConnectionLike
    {
        let res: Result<Vec<redis::Value>, RedisError> = 
            redis::cmd("GRAPH.QUERY").arg("Main").arg(query).query_async(con).await;
        let out = if let Ok(_res) = res {
            Ok(())
        } else { Err(QueryError::Query) };
        out
    }
}

#[async_trait]
pub trait QuerunRedisGraph: Querio {
    async fn querun_async_json<C>(con: &mut C, native_a: &Self::QuerioInputA, native_b: &Self::QuerioInputB, vars: &Self::QuerioVariable) -> Result<String,QueryError> 
        where C: aio::ConnectionLike + Send, Self::QuerioInputA: Send + Sync, Self::QuerioInputB: Send + Sync, Self::QuerioVariable: Send + Sync
    {
        QuerunRedisGraphExe::result_async(con,Self::querio(&native_a, &native_b, &vars)).await
    }
    async fn qrun_async_json<C>(con: &mut C, native_a: <<Self as Querio>::QuerioInputA as IntupleStruct>::Intuple, native_b: <<Self as Querio>::QuerioInputB as IntupleStruct>::Intuple, vars: <<Self as Querio>::QuerioVariable as IntupleStruct>::Intuple) -> Result<String,QueryError> 
        where C: aio::ConnectionLike + Send, <<Self as Querio>::QuerioInputA as IntupleStruct>::Intuple: Send + Sync, <<Self as Querio>::QuerioInputB as IntupleStruct>::Intuple: Send + Sync, <<Self as Querio>::QuerioVariable as IntupleStruct>::Intuple: Send + Sync
    {
        QuerunRedisGraphExe::result_async(con,Self::qrio(native_a,native_b,vars)).await
    }
}