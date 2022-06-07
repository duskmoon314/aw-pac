#[doc = "USBCMD register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "EHCI USB Command Register"]
pub mod usbcmd;
#[doc = "USBSTS register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "EHCI USB Status Register"]
pub mod usbsts;
#[doc = "USBINTR register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "EHCI USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "FRINDEX register accessor: an alias for `Reg<FRINDEX_SPEC>`"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "EHCI Frame Index Register"]
pub mod frindex;
#[doc = "CTRLDSSEGMENT register accessor: an alias for `Reg<CTRLDSSEGMENT_SPEC>`"]
pub type CTRLDSSEGMENT = crate::Reg<ctrldssegment::CTRLDSSEGMENT_SPEC>;
#[doc = "EHCI 4G Segment Selector Register"]
pub mod ctrldssegment;
#[doc = "PERIODICLISTBASE register accessor: an alias for `Reg<PERIODICLISTBASE_SPEC>`"]
pub type PERIODICLISTBASE = crate::Reg<periodiclistbase::PERIODICLISTBASE_SPEC>;
#[doc = "EHCI Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "ASYNCLISTADDR register accessor: an alias for `Reg<ASYNCLISTADDR_SPEC>`"]
pub type ASYNCLISTADDR = crate::Reg<asynclistaddr::ASYNCLISTADDR_SPEC>;
#[doc = "EHCI Current Asynchronous List Address Register"]
pub mod asynclistaddr;
#[doc = "CONFIGFLAG register accessor: an alias for `Reg<CONFIGFLAG_SPEC>`"]
pub type CONFIGFLAG = crate::Reg<configflag::CONFIGFLAG_SPEC>;
#[doc = "EHCI Configure Flag Register"]
pub mod configflag;
#[doc = "PORTSC register accessor: an alias for `Reg<PORTSC_SPEC>`"]
pub type PORTSC = crate::Reg<portsc::PORTSC_SPEC>;
#[doc = "EHCI Port Status/Control Register"]
pub mod portsc;
