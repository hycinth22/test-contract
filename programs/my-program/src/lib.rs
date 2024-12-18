use anchor_lang::prelude::*;

security_txt! {
    // Required fields
    name: "Example",
    project_url: "https://github.com/hycinth22/test-contract",
    contacts: "xxx",
    policy: "ttt",
}

declare_id!("fGJbVTiPPZYymt2uduyFbxFnRLYMXJZnvcLPo1LmXJ3");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
