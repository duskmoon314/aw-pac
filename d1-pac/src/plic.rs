#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x400 - Interrupt Priority Register"]
    pub prio: [PRIO; 256],
    _reserved1: [u8; 0x0c00],
    #[doc = "0x1000..0x1024 - Interrupt Pending Register"]
    pub ip: [IP; 9],
    _reserved2: [u8; 0x0fdc],
    #[doc = "0x2000..0x2024 - Machine Mode Interrupt Enable Register"]
    pub mie: [MIE; 9],
    _reserved3: [u8; 0x5c],
    #[doc = "0x2080..0x20a4 - Supervisor Mode Interrupt Enable Register"]
    pub sie: [SIE; 9],
    _reserved4: [u8; 0x001f_df58],
    #[doc = "0x1ffffc - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x200000 - Machine Mode Priority Threshold Register"]
    pub mth: MTH,
    #[doc = "0x200004 - Machine Mode Claim/Complete Register"]
    pub mclaim: MCLAIM,
    _reserved7: [u8; 0x0ff8],
    #[doc = "0x201000 - Supervisor Mode Priority Threshold Register"]
    pub sth: STH,
    #[doc = "0x201004 - Supervisor Mode Claim/Complete Register"]
    pub sclaim: SCLAIM,
}
#[doc = "prio (rw) register accessor: an alias for `Reg<PRIO_SPEC>`"]
pub type PRIO = crate::Reg<prio::PRIO_SPEC>;
#[doc = "Interrupt Priority Register"]
pub mod prio;
#[doc = "ip (rw) register accessor: an alias for `Reg<IP_SPEC>`"]
pub type IP = crate::Reg<ip::IP_SPEC>;
#[doc = "Interrupt Pending Register"]
pub mod ip;
#[doc = "mie (rw) register accessor: an alias for `Reg<MIE_SPEC>`"]
pub type MIE = crate::Reg<mie::MIE_SPEC>;
#[doc = "Machine Mode Interrupt Enable Register"]
pub mod mie;
#[doc = "sie (rw) register accessor: an alias for `Reg<SIE_SPEC>`"]
pub type SIE = crate::Reg<sie::SIE_SPEC>;
#[doc = "Supervisor Mode Interrupt Enable Register"]
pub mod sie;
#[doc = "ctrl (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "mth (rw) register accessor: an alias for `Reg<MTH_SPEC>`"]
pub type MTH = crate::Reg<mth::MTH_SPEC>;
#[doc = "Machine Mode Priority Threshold Register"]
pub mod mth;
#[doc = "mclaim (rw) register accessor: an alias for `Reg<MCLAIM_SPEC>`"]
pub type MCLAIM = crate::Reg<mclaim::MCLAIM_SPEC>;
#[doc = "Machine Mode Claim/Complete Register"]
pub mod mclaim;
#[doc = "sth (rw) register accessor: an alias for `Reg<STH_SPEC>`"]
pub type STH = crate::Reg<sth::STH_SPEC>;
#[doc = "Supervisor Mode Priority Threshold Register"]
pub mod sth;
#[doc = "sclaim (rw) register accessor: an alias for `Reg<SCLAIM_SPEC>`"]
pub type SCLAIM = crate::Reg<sclaim::SCLAIM_SPEC>;
#[doc = "Supervisor Mode Claim/Complete Register"]
pub mod sclaim;
