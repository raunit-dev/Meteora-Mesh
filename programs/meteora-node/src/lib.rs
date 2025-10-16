use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::FetchPrice;

declare_id!("9cTPBKLTSsrDNWNHFzkjctRQKPccE5MwCd51C9DTffF9");

#[program]
pub mod meteora_node {
    use super::*;

    pub fn fetch_price(ctx: Context<FetchPrice>, feed_id: String) -> Result<()> {
        FetchPrice::fetch_price_handler(ctx, &feed_id)
    }
}

