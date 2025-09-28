use anchor_lang::prelude::*;


#[account]
#[derive(Debug,Default)] //traits 
pub struct AmmConfig{
    //PDA bump for AmmConfigAccount
    pub bump : u8,
    //Program authority to udpate AmmConfig
    pub authority: Pubkey,

    pub trade_fee_rate: u32,

    pub protocol_fee_rate: u32,

    pub tick_spacing: u16,

    pub fund_fee_rate: u32,

    //provide reserved bytes for future upgrades without breaking account layout
    pub padding:[u64; 4],
}

//Account layout
impl AmmConfig{
    pub const LEN: usize = 8 //
    + 1 //bump 
    + 32 //authority
    + 4 //trade_fee_rate
    + 4 //protocol_fee_rate
    + 2 //tick_spacing
    + 4 //fund_fee_rate
    + 8 * 4; //padding 

    //Return the number of bytes required to allocate this account 
    pub const fn space() -> usize {
        Self::LEN
    }
}