#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_CAPABILITY {
    #[doc = "0x00 - EHCI Identification Register"]
    pub caplength: CAPLENGTH,
    #[doc = "0x02 - EHCI Host Interface Version Number Register"]
    pub hciversion: HCIVERSION,
    #[doc = "0x04 - EHCI Host Control Structural Parameter Register"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x08 - EHCI Host Controller Capability Parameters Register"]
    pub hccparams: HCCPARAMS,
    #[doc = "0x0c - EHCI Companion Port Route Description"]
    pub hcsp_portroute: HCSP_PORTROUTE,
}
#[doc = "caplength (r) register accessor: an alias for `Reg<CAPLENGTH_SPEC>`"]
pub type CAPLENGTH = crate::Reg<caplength::CAPLENGTH_SPEC>;
#[doc = "EHCI Identification Register"]
pub mod caplength;
#[doc = "hciversion (r) register accessor: an alias for `Reg<HCIVERSION_SPEC>`"]
pub type HCIVERSION = crate::Reg<hciversion::HCIVERSION_SPEC>;
#[doc = "EHCI Host Interface Version Number Register"]
pub mod hciversion;
#[doc = "hcsparams (r) register accessor: an alias for `Reg<HCSPARAMS_SPEC>`"]
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
#[doc = "EHCI Host Control Structural Parameter Register"]
pub mod hcsparams;
#[doc = "hccparams (r) register accessor: an alias for `Reg<HCCPARAMS_SPEC>`"]
pub type HCCPARAMS = crate::Reg<hccparams::HCCPARAMS_SPEC>;
#[doc = "EHCI Host Controller Capability Parameters Register"]
pub mod hccparams;
#[doc = "hcsp_portroute (r) register accessor: an alias for `Reg<HCSP_PORTROUTE_SPEC>`"]
pub type HCSP_PORTROUTE = crate::Reg<hcsp_portroute::HCSP_PORTROUTE_SPEC>;
#[doc = "EHCI Companion Port Route Description"]
pub mod hcsp_portroute;
