use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct Script {
    keys: Vec<String>,
    args: Vec<String>,
    script: String,
}

impl Script {
    pub async fn invoke_async<T>(
        &self,
        con: &mut impl redis::aio::ConnectionLike,
    ) -> redis::RedisResult<T>
    where
        T: redis::FromRedisValue,
    {
        debug!("Invoking script:\n{}", self.script);
        let script = redis::Script::new(&self.script);

        let mut invocation = script.prepare_invoke();

        for key in self.keys.iter() {
            debug!("Adding key: {}", key);
            invocation.key(key);
        }

        for arg in self.args.iter() {
            debug!("Adding arg: {}", arg);
            invocation.arg(arg);
        }

        invocation.invoke_async(con).await
    }
}
