#[doc = r"Register block"]
#[repr(C)]
pub struct HCI_CONTROLLER_PHY_INTERFACE {
    #[doc = "0x00 - HCI Interface Register"]
    pub hci_interface: HCI_INTERFACE,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - HCI Control Register"]
    pub hci_ctrl3: HCI_CTRL3,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - PHY Control Register"]
    pub phy_control: PHY_CONTROL,
    _reserved3: [u8; 0x10],
    #[doc = "0x24 - PHY Status Register"]
    pub phy_status: PHY_STATUS,
    #[doc = "0x28 - HCI SIE Port Disable Control Register"]
    pub hci_sie_port_disable_control: HCI_SIE_PORT_DISABLE_CONTROL,
}
#[doc = "hci_interface (rw) register accessor: an alias for `Reg<HCI_INTERFACE_SPEC>`"]
pub type HCI_INTERFACE = crate::Reg<hci_interface::HCI_INTERFACE_SPEC>;
#[doc = "HCI Interface Register"]
pub mod hci_interface;
#[doc = "hci_ctrl3 (rw) register accessor: an alias for `Reg<HCI_CTRL3_SPEC>`"]
pub type HCI_CTRL3 = crate::Reg<hci_ctrl3::HCI_CTRL3_SPEC>;
#[doc = "HCI Control Register"]
pub mod hci_ctrl3;
#[doc = "phy_control (rw) register accessor: an alias for `Reg<PHY_CONTROL_SPEC>`"]
pub type PHY_CONTROL = crate::Reg<phy_control::PHY_CONTROL_SPEC>;
#[doc = "PHY Control Register"]
pub mod phy_control;
#[doc = "phy_status (r) register accessor: an alias for `Reg<PHY_STATUS_SPEC>`"]
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
#[doc = "PHY Status Register"]
pub mod phy_status;
#[doc = "hci_sie_port_disable_control (rw) register accessor: an alias for `Reg<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>`"]
pub type HCI_SIE_PORT_DISABLE_CONTROL =
    crate::Reg<hci_sie_port_disable_control::HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
#[doc = "HCI SIE Port Disable Control Register"]
pub mod hci_sie_port_disable_control;
