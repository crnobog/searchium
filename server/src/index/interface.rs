use tokio::sync::{mpsc, oneshot};

use super::{CommandError, CommandResult, DynCommand, IndexServer};
use crate::gen::*;

pub fn new(dyncommand_tx : mpsc::Sender<DynCommand>) -> IndexInterface {
    IndexInterface{ dyncommand_tx }
}

pub struct IndexInterface {
    dyncommand_tx: mpsc::Sender<DynCommand>,
}

impl IndexInterface {
    pub async fn get_database_details(&self) -> CommandResult<DatabaseDetailsResponse> {
        let (tx, rx) = oneshot::channel();
        let body = |s: &mut IndexServer| {
            let details = s.get_database_details_internal();
            tx.send(details).map_err(|_| {
                CommandError::InternalError(
                    "Failed to return result on oneshot channel".to_string(),
                )
            })?;
            Ok(())
        };
        self.dyncommand_tx.send(Box::new(body)).await?;
        rx.await?
    }
}