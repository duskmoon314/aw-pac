#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_ROOT_HUB_PARTITION {
    #[doc = "0x00 - OHCI Root Hub Descriptor Register A"]
    pub hc_rh_descriptor_a: HC_RH_DESCRIPTOR_A,
    #[doc = "0x04 - OHCI Root Hub Descriptor Register B"]
    pub hc_rh_descriptor_b: HC_RH_DESCRIPTOR_B,
    #[doc = "0x08 - OHCI Root Hub Status Register"]
    pub hc_rh_status: HC_RH_STATUS,
    #[doc = "0x0c - OHCI Root Hub Port Status Register"]
    pub hc_rh_port_status: HC_RH_PORT_STATUS,
}
#[doc = "hc_rh_descriptor_a (rw) register accessor: an alias for `Reg<HC_RH_DESCRIPTOR_A_SPEC>`"]
pub type HC_RH_DESCRIPTOR_A = crate::Reg<hc_rh_descriptor_a::HC_RH_DESCRIPTOR_A_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register A"]
pub mod hc_rh_descriptor_a;
#[doc = "hc_rh_descriptor_b (rw) register accessor: an alias for `Reg<HC_RH_DESCRIPTOR_B_SPEC>`"]
pub type HC_RH_DESCRIPTOR_B = crate::Reg<hc_rh_descriptor_b::HC_RH_DESCRIPTOR_B_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register B"]
pub mod hc_rh_descriptor_b;
#[doc = "hc_rh_status (rw) register accessor: an alias for `Reg<HC_RH_STATUS_SPEC>`"]
pub type HC_RH_STATUS = crate::Reg<hc_rh_status::HC_RH_STATUS_SPEC>;
#[doc = "OHCI Root Hub Status Register"]
pub mod hc_rh_status;
#[doc = "hc_rh_port_status (rw) register accessor: an alias for `Reg<HC_RH_PORT_STATUS_SPEC>`"]
pub type HC_RH_PORT_STATUS = crate::Reg<hc_rh_port_status::HC_RH_PORT_STATUS_SPEC>;
#[doc = "OHCI Root Hub Port Status Register"]
pub mod hc_rh_port_status;
