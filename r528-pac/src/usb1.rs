#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_e: [u8; 0x08],
    #[doc = "0x08 - EHCI Host Control Capability Parameter Register"]
    pub e_hccparams: crate::Reg<e_hccparams::E_HCCPARAMS_SPEC>,
    #[doc = "0x0c - EHCI Companion Port Route Description"]
    pub e_hcspportroute: crate::Reg<e_hcspportroute::E_HCSPPORTROUTE_SPEC>,
    #[doc = "0x10 - EHCI USB Command Register"]
    pub e_usbcmd: crate::Reg<e_usbcmd::E_USBCMD_SPEC>,
    #[doc = "0x14 - EHCI USB Status Register"]
    pub e_usbsts: crate::Reg<e_usbsts::E_USBSTS_SPEC>,
    #[doc = "0x18 - EHCI USB Interrupt Enable Register"]
    pub e_usbintr: crate::Reg<e_usbintr::E_USBINTR_SPEC>,
    #[doc = "0x1c - EHCI USB Frame Index Register"]
    pub e_frindex: crate::Reg<e_frindex::E_FRINDEX_SPEC>,
    #[doc = "0x20 - EHCI 4G Segment Selector Register"]
    pub e_ctrldssegment: crate::Reg<e_ctrldssegment::E_CTRLDSSEGMENT_SPEC>,
    #[doc = "0x24 - EHCI Frame List Base Address Register"]
    pub e_periodiclistbase: crate::Reg<e_periodiclistbase::E_PERIODICLISTBASE_SPEC>,
    #[doc = "0x28 - EHCI Next Asynchronous List Address Register"]
    pub e_asynclistaddr: crate::Reg<e_asynclistaddr::E_ASYNCLISTADDR_SPEC>,
    _reserved10: [u8; 0x24],
    #[doc = "0x50 - EHCI Configured Flag Register"]
    pub e_configflag: crate::Reg<e_configflag::E_CONFIGFLAG_SPEC>,
    #[doc = "0x54 - EHCI Port Status/Control Register"]
    pub e_portsc: crate::Reg<e_portsc::E_PORTSC_SPEC>,
    _reserved12: [u8; 0x03ac],
    #[doc = "0x404 - OHCI Control Register"]
    pub o_hc_control: crate::Reg<o_hc_control::O_HCCONTROL_SPEC>,
    #[doc = "0x408 - OHCI Command Status Register"]
    pub o_hc_command_status: crate::Reg<o_hc_command_status::O_HCCOMMANDSTATUS_SPEC>,
    #[doc = "0x40c - OHCI Interrupt Status Register"]
    pub o_hc_interrupt_status: crate::Reg<o_hc_interrupt_status::O_HCINTERRUPTSTATUS_SPEC>,
    #[doc = "0x410 - OHCI Interrupt Enable Register"]
    pub o_hc_interrupt_enable: crate::Reg<o_hc_interrupt_enable::O_HCINTERRUPTENABLE_SPEC>,
    #[doc = "0x414 - OHCI Interrupt Disable Register"]
    pub o_hc_interrupt_disable: crate::Reg<o_hc_interrupt_disable::O_HCINTERRUPTDISABLE_SPEC>,
    #[doc = "0x418 - OHCI HCCA Base"]
    pub o_hc_hcca: crate::Reg<o_hc_hcca::O_HCHCCA_SPEC>,
    #[doc = "0x41c - OHCI Period Current ED Base"]
    pub o_hc_period_current_ed: crate::Reg<o_hc_period_current_ed::O_HCPERIODCURRENTED_SPEC>,
    #[doc = "0x420 - OHCI Control Head ED Base"]
    pub o_hc_control_head_ed: crate::Reg<o_hc_control_head_ed::O_HCCONTROLHEADED_SPEC>,
    #[doc = "0x424 - OHCI Control Current ED Base"]
    pub o_hc_control_current_ed: crate::Reg<o_hc_control_current_ed::O_HCCONTROLCURRENTED_SPEC>,
    #[doc = "0x428 - OHCI Bulk Head ED Base"]
    pub o_hc_bulk_head_ed: crate::Reg<o_hc_bulk_head_ed::O_HCBULKHEADED_SPEC>,
    #[doc = "0x42c - OHCI Bulk Current ED Base"]
    pub o_hc_bulk_current_ed: crate::Reg<o_hc_bulk_current_ed::O_HCBULKCURRENTED_SPEC>,
    #[doc = "0x430 - OHCI Done Head Base"]
    pub o_hc_done_head: crate::Reg<o_hc_done_head::O_HCDONEHEAD_SPEC>,
    #[doc = "0x434 - OHCI Frame Interval Register"]
    pub o_hc_fm_interval: crate::Reg<o_hc_fm_interval::O_HCFMINTERVAL_SPEC>,
    #[doc = "0x438 - OHCI Frame Remaining Register"]
    pub o_hc_fm_remaining: crate::Reg<o_hc_fm_remaining::O_HCFMREMAINING_SPEC>,
    #[doc = "0x43c - OHCI Frame Number Register"]
    pub o_hc_fm_number: crate::Reg<o_hc_fm_number::O_HCFMNUMBER_SPEC>,
    #[doc = "0x440 - OHCI Periodic Start Register"]
    pub o_hc_perioddic_start: crate::Reg<o_hc_perioddic_start::O_HCPERIODDICSTART_SPEC>,
    #[doc = "0x444 - OHCI LS Threshold Register"]
    pub o_hc_lsthreshold: crate::Reg<o_hc_lsthreshold::O_HCLSTHRESHOLD_SPEC>,
    #[doc = "0x448 - OHCI Root Hub Descriptor Register A"]
    pub o_hc_rh_descriptor_a: crate::Reg<o_hc_rh_descriptor_a::O_HCRHDESCRIPTORA_SPEC>,
    #[doc = "0x44c - OHCI Root Hub Descriptor Register B"]
    pub o_hc_rh_desriptor_b: crate::Reg<o_hc_rh_desriptor_b::O_HCRHDESRIPTORB_SPEC>,
    #[doc = "0x450 - OHCI Root Hub Status Register"]
    pub o_hc_rh_status: crate::Reg<o_hc_rh_status::O_HCRHSTATUS_SPEC>,
    #[doc = "0x454 - OHCI Root Hub Port Status Register"]
    pub o_hc_rh_port_status: crate::Reg<o_hc_rh_port_status::O_HCRHPORTSTATUS_SPEC>,
    _reserved33: [u8; 0x03a8],
    #[doc = "0x800 - HCI Interface Register"]
    pub hci_interface: crate::Reg<hci_interface::HCI_INTERFACE_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x808 - HCI Control Register"]
    pub hci_ctrl3: crate::Reg<hci_ctrl3::HCI_CTRL3_SPEC>,
    _reserved35: [u8; 0x04],
    #[doc = "0x810 - PHY Control Register"]
    pub phy_control: crate::Reg<phy_control::PHY_CONTROL_SPEC>,
    _reserved36: [u8; 0x10],
    #[doc = "0x824 - PHY Status Register"]
    pub phy_status: crate::Reg<phy_status::PHY_STATUS_SPEC>,
    #[doc = "0x828 - HCI SIE Port Disable Control Register"]
    pub hci_sie_port_disable_control:
        crate::Reg<hci_sie_port_disable_control::HCI_SIE_PORT_DISABLE_CONTROL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - EHCI Capability Register Length Register"]
    #[inline(always)]
    pub fn e_caplength(&self) -> &crate::Reg<e_caplength::E_CAPLENGTH_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<e_caplength::E_CAPLENGTH_SPEC>)
        }
    }
    #[doc = "0x02 - EHCI Host Interface Version Number Register"]
    #[inline(always)]
    pub fn e_hciversion(&self) -> &crate::Reg<e_hciversion::E_HCIVERSION_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(2usize)
                as *const crate::Reg<e_hciversion::E_HCIVERSION_SPEC>)
        }
    }
    #[doc = "0x04 - EHCI Host Control Structural Parameter Register"]
    #[inline(always)]
    pub fn e_hcsparams(&self) -> &crate::Reg<e_hcsparams::E_HCSPARAMS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<e_hcsparams::E_HCSPARAMS_SPEC>)
        }
    }
}
#[doc = "E_CAPLENGTH register accessor: an alias for `Reg<E_CAPLENGTH_SPEC>`"]
pub type E_CAPLENGTH = crate::Reg<e_caplength::E_CAPLENGTH_SPEC>;
#[doc = "EHCI Capability Register Length Register"]
pub mod e_caplength;
#[doc = "E_HCIVERSION register accessor: an alias for `Reg<E_HCIVERSION_SPEC>`"]
pub type E_HCIVERSION = crate::Reg<e_hciversion::E_HCIVERSION_SPEC>;
#[doc = "EHCI Host Interface Version Number Register"]
pub mod e_hciversion;
#[doc = "E_HCSPARAMS register accessor: an alias for `Reg<E_HCSPARAMS_SPEC>`"]
pub type E_HCSPARAMS = crate::Reg<e_hcsparams::E_HCSPARAMS_SPEC>;
#[doc = "EHCI Host Control Structural Parameter Register"]
pub mod e_hcsparams;
#[doc = "E_HCCPARAMS register accessor: an alias for `Reg<E_HCCPARAMS_SPEC>`"]
pub type E_HCCPARAMS = crate::Reg<e_hccparams::E_HCCPARAMS_SPEC>;
#[doc = "EHCI Host Control Capability Parameter Register"]
pub mod e_hccparams;
#[doc = "E_HCSPPORTROUTE register accessor: an alias for `Reg<E_HCSPPORTROUTE_SPEC>`"]
pub type E_HCSPPORTROUTE = crate::Reg<e_hcspportroute::E_HCSPPORTROUTE_SPEC>;
#[doc = "EHCI Companion Port Route Description"]
pub mod e_hcspportroute;
#[doc = "E_USBCMD register accessor: an alias for `Reg<E_USBCMD_SPEC>`"]
pub type E_USBCMD = crate::Reg<e_usbcmd::E_USBCMD_SPEC>;
#[doc = "EHCI USB Command Register"]
pub mod e_usbcmd;
#[doc = "E_USBSTS register accessor: an alias for `Reg<E_USBSTS_SPEC>`"]
pub type E_USBSTS = crate::Reg<e_usbsts::E_USBSTS_SPEC>;
#[doc = "EHCI USB Status Register"]
pub mod e_usbsts;
#[doc = "E_USBINTR register accessor: an alias for `Reg<E_USBINTR_SPEC>`"]
pub type E_USBINTR = crate::Reg<e_usbintr::E_USBINTR_SPEC>;
#[doc = "EHCI USB Interrupt Enable Register"]
pub mod e_usbintr;
#[doc = "E_FRINDEX register accessor: an alias for `Reg<E_FRINDEX_SPEC>`"]
pub type E_FRINDEX = crate::Reg<e_frindex::E_FRINDEX_SPEC>;
#[doc = "EHCI USB Frame Index Register"]
pub mod e_frindex;
#[doc = "E_CTRLDSSEGMENT register accessor: an alias for `Reg<E_CTRLDSSEGMENT_SPEC>`"]
pub type E_CTRLDSSEGMENT = crate::Reg<e_ctrldssegment::E_CTRLDSSEGMENT_SPEC>;
#[doc = "EHCI 4G Segment Selector Register"]
pub mod e_ctrldssegment;
#[doc = "E_PERIODICLISTBASE register accessor: an alias for `Reg<E_PERIODICLISTBASE_SPEC>`"]
pub type E_PERIODICLISTBASE = crate::Reg<e_periodiclistbase::E_PERIODICLISTBASE_SPEC>;
#[doc = "EHCI Frame List Base Address Register"]
pub mod e_periodiclistbase;
#[doc = "E_ASYNCLISTADDR register accessor: an alias for `Reg<E_ASYNCLISTADDR_SPEC>`"]
pub type E_ASYNCLISTADDR = crate::Reg<e_asynclistaddr::E_ASYNCLISTADDR_SPEC>;
#[doc = "EHCI Next Asynchronous List Address Register"]
pub mod e_asynclistaddr;
#[doc = "E_CONFIGFLAG register accessor: an alias for `Reg<E_CONFIGFLAG_SPEC>`"]
pub type E_CONFIGFLAG = crate::Reg<e_configflag::E_CONFIGFLAG_SPEC>;
#[doc = "EHCI Configured Flag Register"]
pub mod e_configflag;
#[doc = "E_PORTSC register accessor: an alias for `Reg<E_PORTSC_SPEC>`"]
pub type E_PORTSC = crate::Reg<e_portsc::E_PORTSC_SPEC>;
#[doc = "EHCI Port Status/Control Register"]
pub mod e_portsc;
#[doc = "O_HcControl register accessor: an alias for `Reg<O_HCCONTROL_SPEC>`"]
pub type O_HCCONTROL = crate::Reg<o_hc_control::O_HCCONTROL_SPEC>;
#[doc = "OHCI Control Register"]
pub mod o_hc_control;
#[doc = "O_HcCommandStatus register accessor: an alias for `Reg<O_HCCOMMANDSTATUS_SPEC>`"]
pub type O_HCCOMMANDSTATUS = crate::Reg<o_hc_command_status::O_HCCOMMANDSTATUS_SPEC>;
#[doc = "OHCI Command Status Register"]
pub mod o_hc_command_status;
#[doc = "O_HcInterruptStatus register accessor: an alias for `Reg<O_HCINTERRUPTSTATUS_SPEC>`"]
pub type O_HCINTERRUPTSTATUS = crate::Reg<o_hc_interrupt_status::O_HCINTERRUPTSTATUS_SPEC>;
#[doc = "OHCI Interrupt Status Register"]
pub mod o_hc_interrupt_status;
#[doc = "O_HcInterruptEnable register accessor: an alias for `Reg<O_HCINTERRUPTENABLE_SPEC>`"]
pub type O_HCINTERRUPTENABLE = crate::Reg<o_hc_interrupt_enable::O_HCINTERRUPTENABLE_SPEC>;
#[doc = "OHCI Interrupt Enable Register"]
pub mod o_hc_interrupt_enable;
#[doc = "O_HcInterruptDisable register accessor: an alias for `Reg<O_HCINTERRUPTDISABLE_SPEC>`"]
pub type O_HCINTERRUPTDISABLE = crate::Reg<o_hc_interrupt_disable::O_HCINTERRUPTDISABLE_SPEC>;
#[doc = "OHCI Interrupt Disable Register"]
pub mod o_hc_interrupt_disable;
#[doc = "O_HcHCCA register accessor: an alias for `Reg<O_HCHCCA_SPEC>`"]
pub type O_HCHCCA = crate::Reg<o_hc_hcca::O_HCHCCA_SPEC>;
#[doc = "OHCI HCCA Base"]
pub mod o_hc_hcca;
#[doc = "O_HcPeriodCurrentED register accessor: an alias for `Reg<O_HCPERIODCURRENTED_SPEC>`"]
pub type O_HCPERIODCURRENTED = crate::Reg<o_hc_period_current_ed::O_HCPERIODCURRENTED_SPEC>;
#[doc = "OHCI Period Current ED Base"]
pub mod o_hc_period_current_ed;
#[doc = "O_HcControlHeadED register accessor: an alias for `Reg<O_HCCONTROLHEADED_SPEC>`"]
pub type O_HCCONTROLHEADED = crate::Reg<o_hc_control_head_ed::O_HCCONTROLHEADED_SPEC>;
#[doc = "OHCI Control Head ED Base"]
pub mod o_hc_control_head_ed;
#[doc = "O_HcControlCurrentED register accessor: an alias for `Reg<O_HCCONTROLCURRENTED_SPEC>`"]
pub type O_HCCONTROLCURRENTED = crate::Reg<o_hc_control_current_ed::O_HCCONTROLCURRENTED_SPEC>;
#[doc = "OHCI Control Current ED Base"]
pub mod o_hc_control_current_ed;
#[doc = "O_HcBulkHeadED register accessor: an alias for `Reg<O_HCBULKHEADED_SPEC>`"]
pub type O_HCBULKHEADED = crate::Reg<o_hc_bulk_head_ed::O_HCBULKHEADED_SPEC>;
#[doc = "OHCI Bulk Head ED Base"]
pub mod o_hc_bulk_head_ed;
#[doc = "O_HcBulkCurrentED register accessor: an alias for `Reg<O_HCBULKCURRENTED_SPEC>`"]
pub type O_HCBULKCURRENTED = crate::Reg<o_hc_bulk_current_ed::O_HCBULKCURRENTED_SPEC>;
#[doc = "OHCI Bulk Current ED Base"]
pub mod o_hc_bulk_current_ed;
#[doc = "O_HcDoneHead register accessor: an alias for `Reg<O_HCDONEHEAD_SPEC>`"]
pub type O_HCDONEHEAD = crate::Reg<o_hc_done_head::O_HCDONEHEAD_SPEC>;
#[doc = "OHCI Done Head Base"]
pub mod o_hc_done_head;
#[doc = "O_HcFmInterval register accessor: an alias for `Reg<O_HCFMINTERVAL_SPEC>`"]
pub type O_HCFMINTERVAL = crate::Reg<o_hc_fm_interval::O_HCFMINTERVAL_SPEC>;
#[doc = "OHCI Frame Interval Register"]
pub mod o_hc_fm_interval;
#[doc = "O_HcFmRemaining register accessor: an alias for `Reg<O_HCFMREMAINING_SPEC>`"]
pub type O_HCFMREMAINING = crate::Reg<o_hc_fm_remaining::O_HCFMREMAINING_SPEC>;
#[doc = "OHCI Frame Remaining Register"]
pub mod o_hc_fm_remaining;
#[doc = "O_HcFmNumber register accessor: an alias for `Reg<O_HCFMNUMBER_SPEC>`"]
pub type O_HCFMNUMBER = crate::Reg<o_hc_fm_number::O_HCFMNUMBER_SPEC>;
#[doc = "OHCI Frame Number Register"]
pub mod o_hc_fm_number;
#[doc = "O_HcPerioddicStart register accessor: an alias for `Reg<O_HCPERIODDICSTART_SPEC>`"]
pub type O_HCPERIODDICSTART = crate::Reg<o_hc_perioddic_start::O_HCPERIODDICSTART_SPEC>;
#[doc = "OHCI Periodic Start Register"]
pub mod o_hc_perioddic_start;
#[doc = "O_HcLSThreshold register accessor: an alias for `Reg<O_HCLSTHRESHOLD_SPEC>`"]
pub type O_HCLSTHRESHOLD = crate::Reg<o_hc_lsthreshold::O_HCLSTHRESHOLD_SPEC>;
#[doc = "OHCI LS Threshold Register"]
pub mod o_hc_lsthreshold;
#[doc = "O_HcRhDescriptorA register accessor: an alias for `Reg<O_HCRHDESCRIPTORA_SPEC>`"]
pub type O_HCRHDESCRIPTORA = crate::Reg<o_hc_rh_descriptor_a::O_HCRHDESCRIPTORA_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register A"]
pub mod o_hc_rh_descriptor_a;
#[doc = "O_HcRhDesriptorB register accessor: an alias for `Reg<O_HCRHDESRIPTORB_SPEC>`"]
pub type O_HCRHDESRIPTORB = crate::Reg<o_hc_rh_desriptor_b::O_HCRHDESRIPTORB_SPEC>;
#[doc = "OHCI Root Hub Descriptor Register B"]
pub mod o_hc_rh_desriptor_b;
#[doc = "O_HcRhStatus register accessor: an alias for `Reg<O_HCRHSTATUS_SPEC>`"]
pub type O_HCRHSTATUS = crate::Reg<o_hc_rh_status::O_HCRHSTATUS_SPEC>;
#[doc = "OHCI Root Hub Status Register"]
pub mod o_hc_rh_status;
#[doc = "O_HcRhPortStatus register accessor: an alias for `Reg<O_HCRHPORTSTATUS_SPEC>`"]
pub type O_HCRHPORTSTATUS = crate::Reg<o_hc_rh_port_status::O_HCRHPORTSTATUS_SPEC>;
#[doc = "OHCI Root Hub Port Status Register"]
pub mod o_hc_rh_port_status;
#[doc = "HCI_Interface register accessor: an alias for `Reg<HCI_INTERFACE_SPEC>`"]
pub type HCI_INTERFACE = crate::Reg<hci_interface::HCI_INTERFACE_SPEC>;
#[doc = "HCI Interface Register"]
pub mod hci_interface;
#[doc = "HCI_CTRL3 register accessor: an alias for `Reg<HCI_CTRL3_SPEC>`"]
pub type HCI_CTRL3 = crate::Reg<hci_ctrl3::HCI_CTRL3_SPEC>;
#[doc = "HCI Control Register"]
pub mod hci_ctrl3;
#[doc = "PHY_Control register accessor: an alias for `Reg<PHY_CONTROL_SPEC>`"]
pub type PHY_CONTROL = crate::Reg<phy_control::PHY_CONTROL_SPEC>;
#[doc = "PHY Control Register"]
pub mod phy_control;
#[doc = "PHY_STATUS register accessor: an alias for `Reg<PHY_STATUS_SPEC>`"]
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
#[doc = "PHY Status Register"]
pub mod phy_status;
#[doc = "HCI_SIE_PORT_DISABLE_CONTROL register accessor: an alias for `Reg<HCI_SIE_PORT_DISABLE_CONTROL_SPEC>`"]
pub type HCI_SIE_PORT_DISABLE_CONTROL =
    crate::Reg<hci_sie_port_disable_control::HCI_SIE_PORT_DISABLE_CONTROL_SPEC>;
#[doc = "HCI SIE Port Disable Control Register"]
pub mod hci_sie_port_disable_control;
