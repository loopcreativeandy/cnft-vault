use anchor_lang::prelude::*;
use solana_program::{pubkey, pubkey::Pubkey};
use spl_account_compression::{
    program::SplAccountCompression, Noop,
};
use mpl_bubblegum::{state::TreeConfig};

declare_id!("CNftyK7T8udPwYRzZUMWzbh79rKrz9a5GwV2wv7iEHpk");

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::id()
    }
}

#[program]
pub mod cnft_vault {

    use super::*;

    pub fn withdraw_cnft(ctx: Context<Withdraw>,
        root: [u8; 32],
        data_hash: [u8; 32],
        creator_hash: [u8; 32],
        nonce: u64,
        index: u32,) -> Result<()> {
        // let's imagine this data was stored inside an account
        let merkle_tree = pubkey!("trezdkTFPKyj4gE9LAJYPpxn8AYVCvM7Mc4JkTb9X5B");
        let leaf_index = 1u32;
        msg!("attempting to send nft {} from tree {}", leaf_index, merkle_tree);
        

        // // assertions
        // assert_pubkey_equal(&merkle_tree, ctx.accounts.merkle_tree.key, ProgramError::Custom(MyError::UnexpectedTree))?;
        // require!(index == leaf_index,MyError::UnexpectedIndex); // todo make them nicer


        // CPI to bubblegum
        // //attempt 1
        // mpl_bubblegum::cpi::transfer(
        //     CpiContext::new_with_signer(
        //         ctx.accounts.bubblegum_program.to_account_info(), 
        //         mpl_bubblegum::cpi::accounts::Transfer{
        //             tree_authority: ctx.accounts.tree_authority.to_account_info(),
        //             leaf_owner: ctx.accounts.leaf_owner.to_account_info(),
        //             leaf_delegate: ctx.accounts.leaf_delegate.to_account_info(),
        //             new_leaf_owner: ctx.accounts.new_leaf_owner.to_account_info(),
        //             merkle_tree: ctx.accounts.merkle_tree.to_account_info(),
        //             log_wrapper: ctx.accounts.log_wrapper.to_account_info(),
        //             compression_program: ctx.accounts.compression_program.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //         }, &[&[b"cNFT-vault", &[*ctx.bumps.get("vault").unwrap()]]]),
        //         root, data_hash, creator_hash, nonce, index)
        
        //attempt 2
        let mut accounts:  Vec<solana_program::instruction::AccountMeta> = vec![
            AccountMeta::new_readonly(ctx.accounts.tree_authority.key(), false),
            AccountMeta::new_readonly(ctx.accounts.leaf_owner.key(), true),
            AccountMeta::new_readonly(ctx.accounts.leaf_delegate.key(), false),
            AccountMeta::new_readonly(ctx.accounts.new_leaf_owner.key(), false),
            AccountMeta::new(ctx.accounts.merkle_tree.key(), false),
            AccountMeta::new_readonly(ctx.accounts.log_wrapper.key(), false),
            AccountMeta::new_readonly(ctx.accounts.compression_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        ];
        
        // add "accounts" (hashes) that make up the merkle proof
        for acc in ctx.remaining_accounts.iter() {
            accounts.push(AccountMeta::new_readonly(acc.key(), false));
            // account_infos.push(acc.to_account_info());
        }
        
        let mut data: Vec<u8> = vec![
            //root, data_hash, creator_hash, nonce, index
        ];
        data.extend(root);
        data.extend(data_hash);
        data.extend(creator_hash);
        data.extend(nonce.to_le_bytes());
        data.extend(index.to_le_bytes());

        let mut account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.tree_authority.to_account_info(),
            ctx.accounts.leaf_owner.to_account_info(),
            ctx.accounts.leaf_delegate.to_account_info(),
            ctx.accounts.new_leaf_owner.to_account_info(),
            ctx.accounts.merkle_tree.to_account_info(),
            ctx.accounts.log_wrapper.to_account_info(),
            ctx.accounts.compression_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.remaining_accounts[0].to_account_info(),
            ctx.remaining_accounts[1].to_account_info(),
        ];
        // account_infos.extend(ctx.remaining_accounts.iter().map(|&a| a.to_account_info()).collect::<Vec<AccountInfo>>());
        
        msg!("manual cpi call ");
        solana_program::program::invoke_signed(
        & solana_program::instruction::Instruction {
            program_id: ctx.accounts.bubblegum_program.key(),
            accounts: accounts,
            data: data,
        },
        &account_infos[..],
        // &[
        //     ctx.accounts.tree_authority.to_account_info().clone(),
        //     ctx.accounts.leaf_owner.to_account_info().clone(),
        //     ctx.accounts.leaf_delegate.to_account_info().clone(),
        //     ctx.accounts.new_leaf_owner.to_account_info().clone(),
        //     ctx.accounts.merkle_tree.to_account_info().clone(),
        //     ctx.accounts.log_wrapper.to_account_info().clone(),
        //     ctx.accounts.compression_program.to_account_info().clone(),
        //     ctx.accounts.system_program.to_account_info().clone(),
        //     ctx.remaining_accounts[0].to_account_info().clone(),
        //     ctx.remaining_accounts[1].to_account_info().clone(),
        // ],
        &[&[b"cNFT-vault", &[*ctx.bumps.get("leaf_owner").unwrap()]]])
        .map_err(Into::into)

        
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump, 
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is neither written to nor read from.
    pub tree_authority: Account<'info, TreeConfig>,
    
    #[account(
        seeds = [b"cNFT-vault"],
        bump,
    )]
    /// CHECK: This account doesnt even exist (it is just the pda to sign)
    pub leaf_owner: UncheckedAccount<'info>, // sender (the vault in our case)
    /// CHECK: This account is chekced in the instruction
    pub leaf_delegate: UncheckedAccount<'info>,
    /// CHECK: This account is neither written to nor read from.
    pub new_leaf_owner: UncheckedAccount<'info>, // receiver
    #[account(mut)]
    /// CHECK: This account is modified in the downstream program
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

pub struct TransferWithProof<'info>  {
    pub tree_authority: AccountInfo<'info>,
    pub leaf_owner: AccountInfo<'info>,
    pub leaf_delegate: AccountInfo<'info>,
    pub new_leaf_owner: AccountInfo<'info>,
    pub merkle_tree: AccountInfo<'info>,
    pub log_wrapper: AccountInfo<'info>,
    pub compression_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[error_code]
pub enum MyError {
    #[msg("Wrong tree")]
    UnexpectedTree,
    #[msg("Wrong index")]
    UnexpectedIndex,
}
