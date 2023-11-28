#[doc = r"Register block"]
#[repr(C)]
pub struct EHCI_OPERATIONAL {
    usbcmd: USBCMD,
    _reserved1: [u8; 0x04],
    usbintr: USBINTR,
    frindex: FRINDEX,
    ctrldssegment: CTRLDSSEGMENT,
    _reserved_4_usbsts: [u8; 0x04],
    asynclistaddr: ASYNCLISTADDR,
    _reserved6: [u8; 0x24],
    configflag: CONFIGFLAG,
    portsc: PORTSC,
}
impl EHCI_OPERATIONAL {
    #[doc = "0x00 - EHCI USB Command Register"]
    #[inline(always)]
    pub const fn usbcmd(&self) -> &USBCMD {
        &self.usbcmd
    }
    #[doc = "0x08 - EHCI USB Interrupt Enable Register"]
    #[inline(always)]
    pub const fn usbintr(&self) -> &USBINTR {
        &self.usbintr
    }
    #[doc = "0x0c - EHCI Frame Index Register"]
    #[inline(always)]
    pub const fn frindex(&self) -> &FRINDEX {
        &self.frindex
    }
    #[doc = "0x10 - EHCI 4G Segment Selector Register"]
    #[inline(always)]
    pub const fn ctrldssegment(&self) -> &CTRLDSSEGMENT {
        &self.ctrldssegment
    }
    #[doc = "0x14 - EHCI Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x14 - EHCI USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(&self) -> &USBSTS {
        unsafe { &*(self as *const Self).cast::<u8>().add(20).cast() }
    }
    #[doc = "0x18 - EHCI Current Asynchronous List Address Register"]
    #[inline(always)]
    pub const fn asynclistaddr(&self) -> &ASYNCLISTADDR {
        &self.asynclistaddr
    }
    #[doc = "0x40 - EHCI Configure Flag Register"]
    #[inline(always)]
    pub const fn configflag(&self) -> &CONFIGFLAG {
        &self.configflag
    }
    #[doc = "0x44 - EHCI Port Status/Control Register"]
    #[inline(always)]
    pub const fn portsc(&self) -> &PORTSC {
        &self.portsc
    }
}
#[doc = "usbcmd (rw) register accessor: EHCI USB Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcmd`] module"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "EHCI USB Command Register"]
pub mod usbcmd;
#[doc = "usbsts (rw) register accessor: EHCI USB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsts`] module"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "EHCI USB Status Register"]
pub mod usbsts;
#[doc = "usbintr (rw) register accessor: EHCI USB Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintr`] module"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "EHCI USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "frindex (rw) register accessor: EHCI Frame Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frindex`] module"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "EHCI Frame Index Register"]
pub mod frindex;
#[doc = "ctrldssegment (rw) register accessor: EHCI 4G Segment Selector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrldssegment::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrldssegment::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrldssegment`] module"]
pub type CTRLDSSEGMENT = crate::Reg<ctrldssegment::CTRLDSSEGMENT_SPEC>;
#[doc = "EHCI 4G Segment Selector Register"]
pub mod ctrldssegment;
#[doc = "periodiclistbase (rw) register accessor: EHCI Periodic Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periodiclistbase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`periodiclistbase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periodiclistbase`] module"]
pub type PERIODICLISTBASE = crate::Reg<periodiclistbase::PERIODICLISTBASE_SPEC>;
#[doc = "EHCI Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "asynclistaddr (rw) register accessor: EHCI Current Asynchronous List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asynclistaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asynclistaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynclistaddr`] module"]
pub type ASYNCLISTADDR = crate::Reg<asynclistaddr::ASYNCLISTADDR_SPEC>;
#[doc = "EHCI Current Asynchronous List Address Register"]
pub mod asynclistaddr;
#[doc = "configflag (rw) register accessor: EHCI Configure Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configflag`] module"]
pub type CONFIGFLAG = crate::Reg<configflag::CONFIGFLAG_SPEC>;
#[doc = "EHCI Configure Flag Register"]
pub mod configflag;
#[doc = "portsc (rw) register accessor: EHCI Port Status/Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portsc`] module"]
pub type PORTSC = crate::Reg<portsc::PORTSC_SPEC>;
#[doc = "EHCI Port Status/Control Register"]
pub mod portsc;
