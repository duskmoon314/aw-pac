#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_CONTROL_STATUS_PARTITION {
    #[doc = "0x00 - OHCI Control Register"]
    pub hc_control: HC_CONTROL,
    #[doc = "0x04 - OHCI Command Status Register"]
    pub hc_command_status: HC_COMMAND_STATUS,
    #[doc = "0x08 - OHCI Interrupt Status Register"]
    pub hc_interrupt_status: HC_INTERRUPT_STATUS,
    #[doc = "0x0c - OHCI Interrupt Enable Register"]
    pub hc_interrupt_enable: HC_INTERRUPT_ENABLE,
    #[doc = "0x10 - OHCI Interrupt Disable Register"]
    pub hc_interrupt_disable: HC_INTERRUPT_DISABLE,
}
#[doc = "hc_control (rw) register accessor: an alias for `Reg<HC_CONTROL_SPEC>`"]
pub type HC_CONTROL = crate::Reg<hc_control::HC_CONTROL_SPEC>;
#[doc = "OHCI Control Register"]
pub mod hc_control;
#[doc = "hc_command_status (rw) register accessor: an alias for `Reg<HC_COMMAND_STATUS_SPEC>`"]
pub type HC_COMMAND_STATUS = crate::Reg<hc_command_status::HC_COMMAND_STATUS_SPEC>;
#[doc = "OHCI Command Status Register"]
pub mod hc_command_status;
#[doc = "hc_interrupt_status (rw) register accessor: an alias for `Reg<HC_INTERRUPT_STATUS_SPEC>`"]
pub type HC_INTERRUPT_STATUS = crate::Reg<hc_interrupt_status::HC_INTERRUPT_STATUS_SPEC>;
#[doc = "OHCI Interrupt Status Register"]
pub mod hc_interrupt_status;
#[doc = "hc_interrupt_enable (rw) register accessor: an alias for `Reg<HC_INTERRUPT_ENABLE_SPEC>`"]
pub type HC_INTERRUPT_ENABLE = crate::Reg<hc_interrupt_enable::HC_INTERRUPT_ENABLE_SPEC>;
#[doc = "OHCI Interrupt Enable Register"]
pub mod hc_interrupt_enable;
#[doc = "hc_interrupt_disable (rw) register accessor: an alias for `Reg<HC_INTERRUPT_DISABLE_SPEC>`"]
pub type HC_INTERRUPT_DISABLE = crate::Reg<hc_interrupt_disable::HC_INTERRUPT_DISABLE_SPEC>;
#[doc = "OHCI Interrupt Disable Register"]
pub mod hc_interrupt_disable;
