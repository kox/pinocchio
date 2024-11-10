use pinocchio::{
    instruction::{Instruction, Signer},
    program::invoke_signed,
    ProgramResult,
};

/// Memo instruction
///
/// ### Accounts
///     None required
/// ### Instruction Data:
///     - UTF 8 encoded
pub struct Memo<'a> {
    /// Memo
    pub input: &'a [u8],
}

impl<'a> Memo<'a> {
    /// Invoke the memo instruction
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        let instruction = Instruction {
            program_id: &crate::ID,
            accounts: &[],
            data: self.input,
        };

        invoke_signed(&instruction, &[], signers)
    }
}
