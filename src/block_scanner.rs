use std::{sync::Arc, time::Duration};

use ethers::prelude::*;
use tokio::time::sleep;

use crate::timestamp_print;
use colored::*;

pub async fn loop_blocks(http_provider: Arc<Provider<Http>>) {
    let mut last_block: U64 = U64::zero();

    loop {
        if let Ok(block) = http_provider.get_block_number().await {
            if block > last_block {
                last_block = block;
                timestamp_print!(
                    Color::White,
                    Some(false),
                    format!("---------- BLOCK: {:?} ----------", block)
                );
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}
