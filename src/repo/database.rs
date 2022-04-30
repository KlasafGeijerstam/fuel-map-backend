use async_trait::async_trait;

#[async_trait]
pub trait DatabaseUtilRepo {
    async fn alive(&self) -> bool;
}
