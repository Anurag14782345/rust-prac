use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Fix the typo in the entrypoint macro
entrypoint!(pi);

fn pi(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Log a message showing the program_id, number of accounts, and instruction data
    msg!(
        "pi: {} : {} accounts, data: {:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
