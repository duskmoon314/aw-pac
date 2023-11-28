#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_ROOT_HUB_PARTITION {
    hc_rh_descriptor_a: HC_RH_DESCRIPTOR_A,
    hc_rh_descriptor_b: HC_RH_DESCRIPTOR_B,
    hc_rh_status: HC_RH_STATUS,
    hc_rh_port_status: HC_RH_PORT_STATUS,
}
impl OHCI_ROOT_HUB_PARTITION {
    #[doc = "0x00 - OHCI Root Hub Descriptor Register A"]
    #[inline(always)]
    pub const fn hc_rh_descriptor_a(&self) -> &HC_RH_DESCRIPTOR_A {
        &self.hc_rh_descriptor_a
    }
    #[doc = "0x04 - OHCI Root Hub Descriptor Register B"]
    #[inline(always)]
    pub const fn hc_rh_descriptor_b(&self) -> &HC_RH_DESCRIPTOR_B {
        &self.hc_rh_descriptor_b
    }
    #[doc = "0x08 - OHCI Root Hub Status Register"]
    #[inline(always)]
    pub const fn hc_rh_status(&self) -> &HC_RH_STATUS {
        &self.hc_rh_status
    }
    #[doc = "0x0c - OHCI Root Hub Port Status Register"]
    #[inline(always)]
    pub const fn hc_rh_port_status(&self) -> &HC_RH_PORT_STATUS {
        &self.hc_rh_port_status
    }
}
#[doc = "hc_rh_descriptor_a (rw) register accessor: OHCI Root Hub Descriptor Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_rh_descriptor_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_rh_descriptor_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_rh_descriptor_a`] module"]
pub type HC_RH_DESCRIPTOR_A = crate::Reg<hc_rh_descriptor_a::HC_RH_DESCRIPTOR_A_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register A"]
pub mod hc_rh_descriptor_a;
#[doc = "hc_rh_descriptor_b (rw) register accessor: OHCI Root Hub Descriptor Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_rh_descriptor_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_rh_descriptor_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_rh_descriptor_b`] module"]
pub type HC_RH_DESCRIPTOR_B = crate::Reg<hc_rh_descriptor_b::HC_RH_DESCRIPTOR_B_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register B"]
pub mod hc_rh_descriptor_b;
#[doc = "hc_rh_status (rw) register accessor: OHCI Root Hub Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_rh_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_rh_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_rh_status`] module"]
pub type HC_RH_STATUS = crate::Reg<hc_rh_status::HC_RH_STATUS_SPEC>;
#[doc = "OHCI Root Hub Status Register"]
pub mod hc_rh_status;
#[doc = "hc_rh_port_status (rw) register accessor: OHCI Root Hub Port Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_rh_port_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_rh_port_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_rh_port_status`] module"]
pub type HC_RH_PORT_STATUS = crate::Reg<hc_rh_port_status::HC_RH_PORT_STATUS_SPEC>;
#[doc = "OHCI Root Hub Port Status Register"]
pub mod hc_rh_port_status;
