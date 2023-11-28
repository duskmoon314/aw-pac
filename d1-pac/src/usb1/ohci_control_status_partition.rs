#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_CONTROL_STATUS_PARTITION {
    hc_control: HC_CONTROL,
    hc_command_status: HC_COMMAND_STATUS,
    hc_interrupt_status: HC_INTERRUPT_STATUS,
    hc_interrupt_enable: HC_INTERRUPT_ENABLE,
    hc_interrupt_disable: HC_INTERRUPT_DISABLE,
}
impl OHCI_CONTROL_STATUS_PARTITION {
    #[doc = "0x00 - OHCI Control Register"]
    #[inline(always)]
    pub const fn hc_control(&self) -> &HC_CONTROL {
        &self.hc_control
    }
    #[doc = "0x04 - OHCI Command Status Register"]
    #[inline(always)]
    pub const fn hc_command_status(&self) -> &HC_COMMAND_STATUS {
        &self.hc_command_status
    }
    #[doc = "0x08 - OHCI Interrupt Status Register"]
    #[inline(always)]
    pub const fn hc_interrupt_status(&self) -> &HC_INTERRUPT_STATUS {
        &self.hc_interrupt_status
    }
    #[doc = "0x0c - OHCI Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hc_interrupt_enable(&self) -> &HC_INTERRUPT_ENABLE {
        &self.hc_interrupt_enable
    }
    #[doc = "0x10 - OHCI Interrupt Disable Register"]
    #[inline(always)]
    pub const fn hc_interrupt_disable(&self) -> &HC_INTERRUPT_DISABLE {
        &self.hc_interrupt_disable
    }
}
#[doc = "hc_control (rw) register accessor: OHCI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_control`] module"]
pub type HC_CONTROL = crate::Reg<hc_control::HC_CONTROL_SPEC>;
#[doc = "OHCI Control Register"]
pub mod hc_control;
#[doc = "hc_command_status (rw) register accessor: OHCI Command Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_command_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_command_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_command_status`] module"]
pub type HC_COMMAND_STATUS = crate::Reg<hc_command_status::HC_COMMAND_STATUS_SPEC>;
#[doc = "OHCI Command Status Register"]
pub mod hc_command_status;
#[doc = "hc_interrupt_status (rw) register accessor: OHCI Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_interrupt_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_interrupt_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_interrupt_status`] module"]
pub type HC_INTERRUPT_STATUS = crate::Reg<hc_interrupt_status::HC_INTERRUPT_STATUS_SPEC>;
#[doc = "OHCI Interrupt Status Register"]
pub mod hc_interrupt_status;
#[doc = "hc_interrupt_enable (rw) register accessor: OHCI Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_interrupt_enable`] module"]
pub type HC_INTERRUPT_ENABLE = crate::Reg<hc_interrupt_enable::HC_INTERRUPT_ENABLE_SPEC>;
#[doc = "OHCI Interrupt Enable Register"]
pub mod hc_interrupt_enable;
#[doc = "hc_interrupt_disable (rw) register accessor: OHCI Interrupt Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_interrupt_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_interrupt_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hc_interrupt_disable`] module"]
pub type HC_INTERRUPT_DISABLE = crate::Reg<hc_interrupt_disable::HC_INTERRUPT_DISABLE_SPEC>;
#[doc = "OHCI Interrupt Disable Register"]
pub mod hc_interrupt_disable;
