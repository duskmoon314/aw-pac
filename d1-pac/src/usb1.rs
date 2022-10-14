#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - EHCI Capability Register"]
    pub ehci_capability: EHCI_CAPABILITY,
    #[doc = "0x10..0x58 - EHCI Operational Register"]
    pub ehci_operational: EHCI_OPERATIONAL,
    _reserved2: [u8; 0x03ac],
    #[doc = "0x404..0x418 - OHCI Control and Status Partition Register"]
    pub ohci_control_status_partition: OHCI_CONTROL_STATUS_PARTITION,
    #[doc = "0x418..0x434 - OHCI Memory Pointer Partition Register"]
    pub ohci_memory_pointer_partition: OHCI_MEMORY_POINTER_PARTITION,
    #[doc = "0x434..0x448 - OHCI Frame Counter Partition Register"]
    pub ohci_frame_counter_partition: OHCI_FRAME_COUNTER_PARTITION,
    #[doc = "0x448..0x458 - OHCI Root Hub Partition Register"]
    pub ohci_root_hub_partition: OHCI_ROOT_HUB_PARTITION,
    _reserved6: [u8; 0x03a8],
    #[doc = "0x800..0x82c - HCI Controller and PHY Interface Register"]
    pub hci_controller_phy_interface: HCI_CONTROLLER_PHY_INTERFACE,
}
#[doc = "EHCI Capability Register"]
pub use self::ehci_capability::EHCI_CAPABILITY;
#[doc = r"Cluster"]
#[doc = "EHCI Capability Register"]
pub mod ehci_capability;
#[doc = "EHCI Operational Register"]
pub use self::ehci_operational::EHCI_OPERATIONAL;
#[doc = r"Cluster"]
#[doc = "EHCI Operational Register"]
pub mod ehci_operational;
#[doc = "OHCI Control and Status Partition Register"]
pub use self::ohci_control_status_partition::OHCI_CONTROL_STATUS_PARTITION;
#[doc = r"Cluster"]
#[doc = "OHCI Control and Status Partition Register"]
pub mod ohci_control_status_partition;
#[doc = "OHCI Memory Pointer Partition Register"]
pub use self::ohci_memory_pointer_partition::OHCI_MEMORY_POINTER_PARTITION;
#[doc = r"Cluster"]
#[doc = "OHCI Memory Pointer Partition Register"]
pub mod ohci_memory_pointer_partition;
#[doc = "OHCI Frame Counter Partition Register"]
pub use self::ohci_frame_counter_partition::OHCI_FRAME_COUNTER_PARTITION;
#[doc = r"Cluster"]
#[doc = "OHCI Frame Counter Partition Register"]
pub mod ohci_frame_counter_partition;
#[doc = "OHCI Root Hub Partition Register"]
pub use self::ohci_root_hub_partition::OHCI_ROOT_HUB_PARTITION;
#[doc = r"Cluster"]
#[doc = "OHCI Root Hub Partition Register"]
pub mod ohci_root_hub_partition;
#[doc = "HCI Controller and PHY Interface Register"]
pub use self::hci_controller_phy_interface::HCI_CONTROLLER_PHY_INTERFACE;
#[doc = r"Cluster"]
#[doc = "HCI Controller and PHY Interface Register"]
pub mod hci_controller_phy_interface;
