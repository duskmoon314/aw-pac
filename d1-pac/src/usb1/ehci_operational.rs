#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_OPERATIONAL {
    #[doc = "0x00 - EHCI USB Command Register"]
    pub usbcmd: USBCMD,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EHCI USB Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x0c - EHCI Frame Index Register"]
    pub frindex: FRINDEX,
    #[doc = "0x10 - EHCI 4G Segment Selector Register"]
    pub ctrldssegment: CTRLDSSEGMENT,
    _reserved_4_usbsts: [u8; 0x04],
    #[doc = "0x18 - EHCI Current Asynchronous List Address Register"]
    pub asynclistaddr: ASYNCLISTADDR,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - EHCI Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x44 - EHCI Port Status/Control Register"]
    pub portsc: PORTSC,
}
impl EHCI_OPERATIONAL {
    #[doc = "0x14 - EHCI Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
    #[doc = "0x14 - EHCI USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(&self) -> &USBSTS {
        unsafe { &*(self as *const Self).cast::<u8>().add(20usize).cast() }
    }
}
#[doc = "usbcmd (rw) register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "EHCI USB Command Register"]
pub mod usbcmd;
#[doc = "usbsts (rw) register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "EHCI USB Status Register"]
pub mod usbsts;
#[doc = "usbintr (rw) register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "EHCI USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "frindex (rw) register accessor: an alias for `Reg<FRINDEX_SPEC>`"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "EHCI Frame Index Register"]
pub mod frindex;
#[doc = "ctrldssegment (rw) register accessor: an alias for `Reg<CTRLDSSEGMENT_SPEC>`"]
pub type CTRLDSSEGMENT = crate::Reg<ctrldssegment::CTRLDSSEGMENT_SPEC>;
#[doc = "EHCI 4G Segment Selector Register"]
pub mod ctrldssegment;
#[doc = "periodiclistbase (rw) register accessor: an alias for `Reg<PERIODICLISTBASE_SPEC>`"]
pub type PERIODICLISTBASE = crate::Reg<periodiclistbase::PERIODICLISTBASE_SPEC>;
#[doc = "EHCI Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "asynclistaddr (rw) register accessor: an alias for `Reg<ASYNCLISTADDR_SPEC>`"]
pub type ASYNCLISTADDR = crate::Reg<asynclistaddr::ASYNCLISTADDR_SPEC>;
#[doc = "EHCI Current Asynchronous List Address Register"]
pub mod asynclistaddr;
#[doc = "configflag (rw) register accessor: an alias for `Reg<CONFIGFLAG_SPEC>`"]
pub type CONFIGFLAG = crate::Reg<configflag::CONFIGFLAG_SPEC>;
#[doc = "EHCI Configure Flag Register"]
pub mod configflag;
#[doc = "portsc (rw) register accessor: an alias for `Reg<PORTSC_SPEC>`"]
pub type PORTSC = crate::Reg<portsc::PORTSC_SPEC>;
#[doc = "EHCI Port Status/Control Register"]
pub mod portsc;
