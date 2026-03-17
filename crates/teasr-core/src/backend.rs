use anyhow::Result;

use crate::types::{CapturedFrame, Interaction};

#[async_trait::async_trait]
pub trait CaptureBackend: Send {
    fn mode_name(&self) -> &'static str;
    async fn setup(&mut self) -> Result<()>;
    async fn execute(&mut self, interaction: &Interaction) -> Result<Vec<CapturedFrame>>;
    async fn snapshot(&mut self) -> Result<CapturedFrame>;
    async fn teardown(&mut self) -> Result<()>;
}
