use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod don_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        let token = &mut ctx.accounts.token_state;
        token.owner = *ctx.accounts.owner.key;
        token.total_supply = total_supply;
        token.marketing_wallet = ctx.accounts.marketing_wallet.key();
        token.liquidity_wallet = ctx.accounts.liquidity_wallet.key();
        token.burn_wallet = ctx.accounts.burn_wallet.key();
        token.team_wallet = ctx.accounts.team_wallet.key();
        token.reserve_wallet = ctx.accounts.reserve_wallet.key();
        token.blacklist = Vec::new();
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let token = &mut ctx.accounts.token_state;
        let sender = &ctx.accounts.sender;

        require!(
            !token.blacklist.contains(&sender.key()),
            CustomError::AddressBlacklisted
        );

        let marketing_fee = amount * 2 / 100;
        let liquidity_fee = amount * 2 / 100;
        let burn_fee = amount * 1 / 100;
        let net_amount = amount - marketing_fee - liquidity_fee - burn_fee;

        **sender.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.marketing_wallet.to_account_info().try_borrow_mut_lamports()? += marketing_fee;
        **ctx.accounts.liquidity_wallet.to_account_info().try_borrow_mut_lamports()? += liquidity_fee;
        **ctx.accounts.burn_wallet.to_account_info().try_borrow_mut_lamports()? += burn_fee;
        **ctx.accounts.receiver.to_account_info().try_borrow_mut_lamports()? += net_amount;

        Ok(())
    }

    pub fn freeze(ctx: Context<Freeze>, address: Pubkey) -> Result<()> {
        let token = &mut ctx.accounts.token_state;
        require!(token.owner == *ctx.accounts.authority.key, CustomError::Unauthorized);
        if !token.blacklist.contains(&address) {
            token.blacklist.push(address);
        }
        Ok(())
    }

    pub fn unfreeze(ctx: Context<Freeze>, address: Pubkey) -> Result<()> {
        let token = &mut ctx.accounts.token_state;
        require!(token.owner == *ctx.accounts.authority.key, CustomError::Unauthorized);
        token.blacklist.retain(|&x| x != address);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = 8 + 32*7 + 8 + (4 + 50 * 32))]
    pub token_state: Account<'info, TokenState>,
    pub marketing_wallet: AccountInfo<'info>,
    pub liquidity_wallet: AccountInfo<'info>,
    pub burn_wallet: AccountInfo<'info>,
    pub team_wallet: AccountInfo<'info>,
    pub reserve_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub token_state: Account<'info, TokenState>,
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub marketing_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub liquidity_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub burn_wallet: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Freeze<'info> {
    #[account(mut)]
    pub token_state: Account<'info, TokenState>,
    pub authority: Signer<'info>,
}

#[account]
pub struct TokenState {
    pub owner: Pubkey,
    pub total_supply: u64,
    pub marketing_wallet: Pubkey,
    pub liquidity_wallet: Pubkey,
    pub burn_wallet: Pubkey,
    pub team_wallet: Pubkey,
    pub reserve_wallet: Pubkey,
    pub blacklist: Vec<Pubkey>,
}

#[error_code]
pub enum CustomError {
    #[msg("This address is blacklisted.")]
    AddressBlacklisted,
    #[msg("You are not authorized.")]
    Unauthorized,
}
