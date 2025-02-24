use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{self, PriceUpdateV2};

declare_id!("2hipD8X5mnXd5gBwKgG6Pkc5tu3UKdT5RSoMQCpW28wk");

const MAXIMUM_PRICE_AGE: u64 = 60;
const SOL_USD_FEED: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

#[program]
pub mod pyth_example {
    use super::*;

    pub fn fetch_price(ctx: Context<FetchPrice>) -> Result<()> {
        let price_feed = &mut ctx.accounts.price_oracle;

        let feed_id: [u8; 32] = price_update::get_feed_id_from_hex(SOL_USD_FEED)?;

        // get_price_no_older_than will fail if the price update is more than MAXIMUM_PRICE_AGE seconds old
        // get_price_no_older_than will fail if the price update is for a different price feed.
        let price =
            price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_PRICE_AGE, &feed_id)?;

        let delta = Clock::get()?.unix_timestamp - price.publish_time;

        // Sample output:
        // The price is (7160106530699 ± 5129162301) * 10^-8
        msg!(
            "The price is ({} ± {}) * 10^{} with a delta of {}s",
            price.price,
            price.conf,
            price.exponent,
            delta,
        );

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Add this account to any instruction Context that needs price data.
    pub price_oracle: Account<'info, PriceUpdateV2>,
}
