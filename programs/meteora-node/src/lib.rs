use anchor_lang::prelude::*;

declare_id!("9cTPBKLTSsrDNWNHFzkjctRQKPccE5MwCd51C9DTffF9");

#[program]
pub mod meteora_node {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
