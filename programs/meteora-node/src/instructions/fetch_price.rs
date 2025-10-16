use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::{price_update::PriceUpdateV2,price_update::get_feed_id_from_hex};

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub price_update_account: Account<'info, PriceUpdateV2>,
}

impl<'info> FetchPrice<'info> {
    pub fn fetch_price_handler(ctx: Context<FetchPrice<'info>>, feed_id: &str) -> Result<()> {

        let price_feed = get_feed_id_from_hex(feed_id)?;

        let max_age = 600;

        let clock = Clock::get()?;

        let price = ctx.accounts.price_update_account.get_price_no_older_than(&clock, max_age, &price_feed)?;

        msg!("Price: {}", price.price);

        Ok(())
    }
}