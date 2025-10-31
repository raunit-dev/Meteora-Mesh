use crate::lb_clmm;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct AddLiquidityByStrategy<'info> {
    /// CHECK: Position account
    #[account(mut)]
    pub position: UncheckedAccount<'info>,

    /// CHECK: LB Pair (DLMM pool)
    #[account(mut)]
    pub lb_pair: UncheckedAccount<'info>,

    /// CHECK: Bin array bitmap extension (optional)
    #[account(mut)]
    pub bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,

    #[account(mut)]
    pub user_token_x: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user_token_y: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: Pool's reserve X
    #[account(mut)]
    pub reserve_x: UncheckedAccount<'info>,

    /// CHECK: Pool's reserve Y
    #[account(mut)]
    pub reserve_y: UncheckedAccount<'info>,

    pub token_x_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_y_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub token_x_program: Interface<'info, TokenInterface>,

    pub token_y_program: Interface<'info, TokenInterface>,

    /// CHECK: PDA with seeds ["__event_authority"]
    #[account(mut)]
    pub event_authority: UncheckedAccount<'info>,

    /// CHECK: DLMM program
    #[account(address = lb_clmm::ID)]
    pub lb_clmm_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddLiquidityByStrategy<'info> {
    pub fn add_liquidity(
        &mut self,
        total_x_amount: u64,
        total_y_amount: u64,
        min_bin_id: i32,
        max_bin_id: i32,
        strategy_type: lb_clmm::types::StrategyType,
        active_id: i32,
        max_slippage: i32,
    ) -> Result<()> {
        let accounts = lb_clmm::cpi::accounts::AddLiquidityByStrategy2 {
            position: self.position.to_account_info(),
            lb_pair: self.lb_pair.to_account_info(),
            bin_array_bitmap_extension: self
                .bin_array_bitmap_extension
                .as_ref()
                .map(|acc| acc.to_account_info()),
            user_token_x: self.user_token_x.to_account_info(),
            user_token_y: self.user_token_y.to_account_info(),
            reserve_x: self.reserve_x.to_account_info(),
            reserve_y: self.reserve_y.to_account_info(),
            token_x_mint: self.token_x_mint.to_account_info(),
            token_y_mint: self.token_y_mint.to_account_info(),
            sender: self.sender.to_account_info(),
            token_x_program: self.token_x_program.to_account_info(),
            token_y_program: self.token_y_program.to_account_info(),
            event_authority: self.event_authority.to_account_info(),
            program: self.lb_clmm_program.to_account_info(),
        };

        let liquidity_parameter = lb_clmm::types::LiquidityParameterByStrategy {
            amount_x: total_x_amount,
            amount_y: total_y_amount,
            active_id,
            max_active_bin_slippage: max_slippage,
            strategy_parameters: lb_clmm::types::StrategyParameters {
                min_bin_id,
                max_bin_id,
                strategy_type,
                parameteres: [0u8; 64],
            },
        };

        let remaining_accounts_info = lb_clmm::types::RemainingAccountsInfo { slices: vec![] };

        let cpi_ctx = CpiContext::new(self.lb_clmm_program.to_account_info(), accounts);

        lb_clmm::cpi::add_liquidity_by_strategy2(
            cpi_ctx,
            liquidity_parameter,
            remaining_accounts_info,
        )?;

        Ok(())
    }
}
