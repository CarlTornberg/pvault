pub struct Instruction<'a, TAccount, TData> {
    pub discriminator: &'a u8,
    pub accounts: TAccount,
    pub data: TData,
}

