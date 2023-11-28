#[doc = r"Register block"]
#[repr(C)]
pub struct HCI_CONTROLLER_PHY_INTERFACE {
    hci_interface: HCI_INTERFACE,
    _reserved1: [u8; 0x04],
    hci_ctrl3: HCI_CTRL3,
    _reserved2: [u8; 0x04],
    phy_control: PHY_CONTROL,
    _reserved3: [u8; 0x10],
    phy_status: PHY_STATUS,
    hci_sie_port_disable_control: HCI_SIE_PORT_DISABLE_CONTROL,
}
impl HCI_CONTROLLER_PHY_INTERFACE {
    #[doc = "0x00 - HCI Interface Register"]
    #[inline(always)]
    pub const fn hci_interface(&self) -> &HCI_INTERFACE {
        &self.hci_interface
    }
    #[doc = "0x08 - HCI Control Register"]
    #[inline(always)]
    pub const fn hci_ctrl3(&self) -> &HCI_CTRL3 {
        &self.hci_ctrl3
    }
    #[doc = "0x10 - PHY Control Register"]
    #[inline(always)]
    pub const fn phy_control(&self) -> &PHY_CONTROL {
        &self.phy_control
    }
    #[doc = "0x24 - PHY Status Register"]
    #[inline(always)]
    pub const fn phy_status(&self) -> &PHY_STATUS {
        &self.phy_status
    }
    #[doc = "0x28 - HCI SIE Port Disable Control Register"]
    #[inline(always)]
    pub const fn hci_sie_port_disable_control(&self) -> &HCI_SIE_PORT_DISABLE_CONTROL {
        &self.hci_sie_port_disable_control
    }
}
#[doc = "hci_interface (rw) register accessor: HCI Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hci_interface::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hci_interface::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hci_interface`] module"]
pub type HCI_INTERFACE = crate::Reg<hci_interface::HCI_INTERFACE_SPEC>;
#[doc = "HCI Interface Register"]
pub mod hci_interface;
#[doc = "hci_ctrl3 (rw) register accessor: HCI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hci_ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hci_ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hci_ctrl3`] module"]
pub type HCI_CTRL3 = crate::Reg<hci_ctrl3::HCI_CTRL3_SPEC>;
#[doc = "HCI Control Register"]
pub mod hci_ctrl3;
#[doc = "phy_control (rw) register accessor: PHY Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_control`] module"]
pub type PHY_CONTROL = crate::Reg<phy_control::PHY_CONTROL_SPEC>;
#[doc = "PHY Control Register"]
pub mod phy_control;
#[doc = "phy_status (r) register accessor: PHY Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_status`] module"]
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
#[doc = "PHY Status Register"]
pub mod phy_status;
#[doc = "hci_sie_port_disable_control (rw) register accessor: HCI SIE Port Disable Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hci_sie_port_disable_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hci_sie_port_disable_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hci_sie_port_disable_control`] module"]
pub type HCI_SIE_PORT_DISABLE_CONTROL =
    crate::Reg<hci_sie_port_disable_control::HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
#[doc = "HCI SIE Port Disable Control Register"]
pub mod hci_sie_port_disable_control;
