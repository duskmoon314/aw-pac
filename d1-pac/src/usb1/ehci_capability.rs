#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_CAPABILITY {
    caplength: CAPLENGTH,
    hciversion: HCIVERSION,
    hcsparams: HCSPARAMS,
    hccparams: HCCPARAMS,
    hcsp_portroute: HCSP_PORTROUTE,
}
impl EHCI_CAPABILITY {
    #[doc = "0x00 - EHCI Identification Register"]
    #[inline(always)]
    pub const fn caplength(&self) -> &CAPLENGTH {
        &self.caplength
    }
    #[doc = "0x02 - EHCI Host Interface Version Number Register"]
    #[inline(always)]
    pub const fn hciversion(&self) -> &HCIVERSION {
        &self.hciversion
    }
    #[doc = "0x04 - EHCI Host Control Structural Parameter Register"]
    #[inline(always)]
    pub const fn hcsparams(&self) -> &HCSPARAMS {
        &self.hcsparams
    }
    #[doc = "0x08 - EHCI Host Controller Capability Parameters Register"]
    #[inline(always)]
    pub const fn hccparams(&self) -> &HCCPARAMS {
        &self.hccparams
    }
    #[doc = "0x0c - EHCI Companion Port Route Description"]
    #[inline(always)]
    pub const fn hcsp_portroute(&self) -> &HCSP_PORTROUTE {
        &self.hcsp_portroute
    }
}
#[doc = "caplength (r) register accessor: EHCI Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caplength::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caplength`] module"]
pub type CAPLENGTH = crate::Reg<caplength::CAPLENGTH_SPEC>;
#[doc = "EHCI Identification Register"]
pub mod caplength;
#[doc = "hciversion (r) register accessor: EHCI Host Interface Version Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hciversion::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hciversion`] module"]
pub type HCIVERSION = crate::Reg<hciversion::HCIVERSION_SPEC>;
#[doc = "EHCI Host Interface Version Number Register"]
pub mod hciversion;
#[doc = "hcsparams (r) register accessor: EHCI Host Control Structural Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsparams::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsparams`] module"]
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
#[doc = "EHCI Host Control Structural Parameter Register"]
pub mod hcsparams;
#[doc = "hccparams (r) register accessor: EHCI Host Controller Capability Parameters Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hccparams::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccparams`] module"]
pub type HCCPARAMS = crate::Reg<hccparams::HCCPARAMS_SPEC>;
#[doc = "EHCI Host Controller Capability Parameters Register"]
pub mod hccparams;
#[doc = "hcsp_portroute (r) register accessor: EHCI Companion Port Route Description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsp_portroute::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsp_portroute`] module"]
pub type HCSP_PORTROUTE = crate::Reg<hcsp_portroute::HCSP_PORTROUTE_SPEC>;
#[doc = "EHCI Companion Port Route Description"]
pub mod hcsp_portroute;
