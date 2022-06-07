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
#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_CAPABILITY {
    #[doc = "0x00 - EHCI Identification Register"]
    pub caplength: crate::Reg<self::ehci_capability::caplength::CAPLENGTH_SPEC>,
    #[doc = "0x02 - EHCI Host Interface Version Number Register"]
    pub hciversion: crate::Reg<self::ehci_capability::hciversion::HCIVERSION_SPEC>,
    #[doc = "0x04 - EHCI Host Control Structural Parameter Register"]
    pub hcsparams: crate::Reg<self::ehci_capability::hcsparams::HCSPARAMS_SPEC>,
    #[doc = "0x08 - EHCI Host Controller Capability Parameters Register"]
    pub hccparams: crate::Reg<self::ehci_capability::hccparams::HCCPARAMS_SPEC>,
    #[doc = "0x0c - EHCI Companion Port Route Description"]
    pub hcsp_portroute: crate::Reg<self::ehci_capability::hcsp_portroute::HCSP_PORTROUTE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "EHCI Capability Register"]
pub mod ehci_capability;
#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_OPERATIONAL {
    #[doc = "0x00 - EHCI USB Command Register"]
    pub usbcmd: crate::Reg<self::ehci_operational::usbcmd::USBCMD_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EHCI USB Interrupt Enable Register"]
    pub usbintr: crate::Reg<self::ehci_operational::usbintr::USBINTR_SPEC>,
    #[doc = "0x0c - EHCI Frame Index Register"]
    pub frindex: crate::Reg<self::ehci_operational::frindex::FRINDEX_SPEC>,
    #[doc = "0x10 - EHCI 4G Segment Selector Register"]
    pub ctrldssegment: crate::Reg<self::ehci_operational::ctrldssegment::CTRLDSSEGMENT_SPEC>,
    _reserved_4_usbsts: [u8; 0x04],
    #[doc = "0x18 - EHCI Current Asynchronous List Address Register"]
    pub asynclistaddr: crate::Reg<self::ehci_operational::asynclistaddr::ASYNCLISTADDR_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - EHCI Configure Flag Register"]
    pub configflag: crate::Reg<self::ehci_operational::configflag::CONFIGFLAG_SPEC>,
    #[doc = "0x44 - EHCI Port Status/Control Register"]
    pub portsc: crate::Reg<self::ehci_operational::portsc::PORTSC_SPEC>,
}
impl EHCI_OPERATIONAL {
    #[doc = "0x14 - EHCI Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub fn periodiclistbase(
        &self,
    ) -> &crate::Reg<self::ehci_operational::periodiclistbase::PERIODICLISTBASE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<
                    self::ehci_operational::periodiclistbase::PERIODICLISTBASE_SPEC,
                >)
        }
    }
    #[doc = "0x14 - EHCI USB Status Register"]
    #[inline(always)]
    pub fn usbsts(&self) -> &crate::Reg<self::ehci_operational::usbsts::USBSTS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<self::ehci_operational::usbsts::USBSTS_SPEC>)
        }
    }
}
#[doc = r"Register block"]
#[doc = "EHCI Operational Register"]
pub mod ehci_operational;
#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_CONTROL_STATUS_PARTITION {
    #[doc = "0x00 - OHCI Control Register"]
    pub hc_control: crate::Reg<self::ohci_control_status_partition::hc_control::HC_CONTROL_SPEC>,
    #[doc = "0x04 - OHCI Command Status Register"]
    pub hc_command_status:
        crate::Reg<self::ohci_control_status_partition::hc_command_status::HC_COMMAND_STATUS_SPEC>,
    #[doc = "0x08 - OHCI Interrupt Status Register"]
    pub hc_interrupt_status: crate::Reg<
        self::ohci_control_status_partition::hc_interrupt_status::HC_INTERRUPT_STATUS_SPEC,
    >,
    #[doc = "0x0c - OHCI Interrupt Enable Register"]
    pub hc_interrupt_enable: crate::Reg<
        self::ohci_control_status_partition::hc_interrupt_enable::HC_INTERRUPT_ENABLE_SPEC,
    >,
    #[doc = "0x10 - OHCI Interrupt Disable Register"]
    pub hc_interrupt_disable: crate::Reg<
        self::ohci_control_status_partition::hc_interrupt_disable::HC_INTERRUPT_DISABLE_SPEC,
    >,
}
#[doc = r"Register block"]
#[doc = "OHCI Control and Status Partition Register"]
pub mod ohci_control_status_partition;
#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_MEMORY_POINTER_PARTITION {
    #[doc = "0x00 - OHCI HCCA Base"]
    pub hc_hcca: crate::Reg<self::ohci_memory_pointer_partition::hc_hcca::HC_HCCA_SPEC>,
    #[doc = "0x04 - OHCI Period Current ED Base"]
    pub hc_period_current_ed: crate::Reg<
        self::ohci_memory_pointer_partition::hc_period_current_ed::HC_PERIOD_CURRENT_ED_SPEC,
    >,
    #[doc = "0x08 - OHCI Control Head ED Base"]
    pub hc_control_head_ed: crate::Reg<
        self::ohci_memory_pointer_partition::hc_control_head_ed::HC_CONTROL_HEAD_ED_SPEC,
    >,
    #[doc = "0x0c - OHCI Control Current ED Base"]
    pub hc_control_current_ed: crate::Reg<
        self::ohci_memory_pointer_partition::hc_control_current_ed::HC_CONTROL_CURRENT_ED_SPEC,
    >,
    #[doc = "0x10 - OHCI Bulk Head ED Base"]
    pub hc_bulk_head_ed:
        crate::Reg<self::ohci_memory_pointer_partition::hc_bulk_head_ed::HC_BULK_HEAD_ED_SPEC>,
    #[doc = "0x14 - OHCI Bulk Current ED Base"]
    pub hc_bulk_current_ed: crate::Reg<
        self::ohci_memory_pointer_partition::hc_bulk_current_ed::HC_BULK_CURRENT_ED_SPEC,
    >,
    #[doc = "0x18 - OHCI Done Head Base"]
    pub hc_done_head:
        crate::Reg<self::ohci_memory_pointer_partition::hc_done_head::HC_DONE_HEAD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "OHCI Memory Pointer Partition Register"]
pub mod ohci_memory_pointer_partition;
#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_FRAME_COUNTER_PARTITION {
    #[doc = "0x00 - OHCI Frame Interval Register"]
    pub hc_fm_interval:
        crate::Reg<self::ohci_frame_counter_partition::hc_fm_interval::HC_FM_INTERVAL_SPEC>,
    #[doc = "0x04 - OHCI Frame Remaining Register"]
    pub hc_fm_remaining:
        crate::Reg<self::ohci_frame_counter_partition::hc_fm_remaining::HC_FM_REMAINING_SPEC>,
    #[doc = "0x08 - OHCI Frame Number Register"]
    pub hc_fm_number:
        crate::Reg<self::ohci_frame_counter_partition::hc_fm_number::HC_FM_NUMBER_SPEC>,
    #[doc = "0x0c - OHCI Periodic Start Register"]
    pub hc_periodic_start:
        crate::Reg<self::ohci_frame_counter_partition::hc_periodic_start::HC_PERIODIC_START_SPEC>,
    #[doc = "0x10 - OHCI LS Threshold Register"]
    pub hc_ls_threshold:
        crate::Reg<self::ohci_frame_counter_partition::hc_ls_threshold::HC_LS_THRESHOLD_SPEC>,
}
#[doc = r"Register block"]
#[doc = "OHCI Frame Counter Partition Register"]
pub mod ohci_frame_counter_partition;
#[doc = r"Register block"]
#[repr(C)]
pub struct OHCI_ROOT_HUB_PARTITION {
    #[doc = "0x00 - OHCI Root Hub Descriptor Register A"]
    pub hc_rh_descriptor_a:
        crate::Reg<self::ohci_root_hub_partition::hc_rh_descriptor_a::HC_RH_DESCRIPTOR_A_SPEC>,
    #[doc = "0x04 - OHCI Root Hub Descriptor Register B"]
    pub hc_rh_descriptor_b:
        crate::Reg<self::ohci_root_hub_partition::hc_rh_descriptor_b::HC_RH_DESCRIPTOR_B_SPEC>,
    #[doc = "0x08 - OHCI Root Hub Status Register"]
    pub hc_rh_status: crate::Reg<self::ohci_root_hub_partition::hc_rh_status::HC_RH_STATUS_SPEC>,
    #[doc = "0x0c - OHCI Root Hub Port Status Register"]
    pub hc_rh_port_status:
        crate::Reg<self::ohci_root_hub_partition::hc_rh_port_status::HC_RH_PORT_STATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "OHCI Root Hub Partition Register"]
pub mod ohci_root_hub_partition;
#[doc = r"Register block"]
#[repr(C)]
pub struct HCI_CONTROLLER_PHY_INTERFACE { # [ doc = "0x00 - HCI Interface Register" ] pub hci_interface : crate :: Reg < self :: hci_controller_phy_interface :: hci_interface :: HCI_INTERFACE_SPEC > , _reserved1 : [ u8 ; 0x04 ] , # [ doc = "0x08 - HCI Control Register" ] pub hci_ctrl3 : crate :: Reg < self :: hci_controller_phy_interface :: hci_ctrl3 :: HCI_CTRL3_SPEC > , _reserved2 : [ u8 ; 0x04 ] , # [ doc = "0x10 - PHY Control Register" ] pub phy_control : crate :: Reg < self :: hci_controller_phy_interface :: phy_control :: PHY_CONTROL_SPEC > , _reserved3 : [ u8 ; 0x10 ] , # [ doc = "0x24 - PHY Status Register" ] pub phy_status : crate :: Reg < self :: hci_controller_phy_interface :: phy_status :: PHY_STATUS_SPEC > , # [ doc = "0x28 - HCI SIE Port Disable Control Register" ] pub hci_sie_port_disable_control : crate :: Reg < self :: hci_controller_phy_interface :: hci_sie_port_disable_control :: HCI_SIE_PORT_DISABLE_CONTROL_SPEC > , }
#[doc = r"Register block"]
#[doc = "HCI Controller and PHY Interface Register"]
pub mod hci_controller_phy_interface;
