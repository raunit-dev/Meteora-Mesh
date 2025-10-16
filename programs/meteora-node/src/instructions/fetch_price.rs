use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::get_feed_id_from_hex;
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub price_update_account: Account<'info, PriceUpdateV2>,
}

impl<'info> FetchPrice<'info> {
    pub fn fetch_price_handler(&mut self, feed_id: &str) -> Result<()> {
        let price_feed = get_feed_id_from_hex(
            "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d",
        )?;

        let max_age = 600;

        let clock = Clock::get()?;

        let price =
            self.price_update_account
                .get_price_no_older_than(&clock, max_age, &price_feed)?;

        msg!("Price: {}", price.price);

        Ok(())
    }
}
