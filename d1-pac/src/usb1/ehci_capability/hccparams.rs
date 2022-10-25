#[doc = "Register `hccparams` reader"]
pub struct R(crate::R<HCCPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `programmable_frame_list_flag` reader - Programmable Frame List Flag\n\nIf this bit is set to a zero, then system software must use a frame list length of 1024 elements with this host controller. The USBCMD register Frame List Size field is a read-only register and should be set to zero.\n\nIf set to 1, then system software can specify and use the frame list in the USBCMD register Frame List Size field to cofigure the host controller.\n\nThe frame list must always aligned on a 4K page boundary. This requirement ensures that the frame list is always physically contiguous."]
pub type PROGRAMMABLE_FRAME_LIST_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `asynchronous_schedule_park_capability` reader - Asynchronous Schedule Park Capability\n\nIf this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule. The feature can be disabled or enabled and set to a specific level by using the Asynchronous Schedule Park Mode Enable and Asynchronous Schedule Park Mode Count fields in the USBCMD register."]
pub type ASYNCHRONOUS_SCHEDULE_PARK_CAPABILITY_R = crate::BitReader<bool>;
#[doc = "Field `isochronous_scheduling_threshold` reader - Isochronous Scheduling Threshold\n\nThis field indicates, relative to the current position of the executing host controller, where software can reliably update the isochronous schedule.\n\nWhen bit\\[7\\] is zero, the value of the least significant 3 bits indicates the number of micro-frames a host controller can hold a set of isochronous data structures(one or more) before flushing the state. When bit\\[7\\] is a one, then host software assumes the host controller may cache an isochronous data structure for an entire frame."]
pub type ISOCHRONOUS_SCHEDULING_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `eecp` reader - EHCI Extended Capabilities Pointer (EECP)\n\nThis optional field indicates the existence of a capabilities list. A value of 00b indicates no extended capabilities are implemented. A non-zero value in this register indicates the offset in PCI configuration space of the first EHCI extended capabiliby. The pointer value must be 40h or greater if implemented to maintain to consistency of the PCI header defined for this calss of device."]
pub type EECP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 1 - Programmable Frame List Flag\n\nIf this bit is set to a zero, then system software must use a frame list length of 1024 elements with this host controller. The USBCMD register Frame List Size field is a read-only register and should be set to zero.\n\nIf set to 1, then system software can specify and use the frame list in the USBCMD register Frame List Size field to cofigure the host controller.\n\nThe frame list must always aligned on a 4K page boundary. This requirement ensures that the frame list is always physically contiguous."]
    #[inline(always)]
    pub fn programmable_frame_list_flag(&self) -> PROGRAMMABLE_FRAME_LIST_FLAG_R {
        PROGRAMMABLE_FRAME_LIST_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park Capability\n\nIf this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule. The feature can be disabled or enabled and set to a specific level by using the Asynchronous Schedule Park Mode Enable and Asynchronous Schedule Park Mode Count fields in the USBCMD register."]
    #[inline(always)]
    pub fn asynchronous_schedule_park_capability(&self) -> ASYNCHRONOUS_SCHEDULE_PARK_CAPABILITY_R {
        ASYNCHRONOUS_SCHEDULE_PARK_CAPABILITY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold\n\nThis field indicates, relative to the current position of the executing host controller, where software can reliably update the isochronous schedule.\n\nWhen bit\\[7\\] is zero, the value of the least significant 3 bits indicates the number of micro-frames a host controller can hold a set of isochronous data structures(one or more) before flushing the state. When bit\\[7\\] is a one, then host software assumes the host controller may cache an isochronous data structure for an entire frame."]
    #[inline(always)]
    pub fn isochronous_scheduling_threshold(&self) -> ISOCHRONOUS_SCHEDULING_THRESHOLD_R {
        ISOCHRONOUS_SCHEDULING_THRESHOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer (EECP)\n\nThis optional field indicates the existence of a capabilities list. A value of 00b indicates no extended capabilities are implemented. A non-zero value in this register indicates the offset in PCI configuration space of the first EHCI extended capabiliby. The pointer value must be 40h or greater if implemented to maintain to consistency of the PCI header defined for this calss of device."]
    #[inline(always)]
    pub fn eecp(&self) -> EECP_R {
        EECP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "EHCI Host Controller Capability Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccparams](index.html) module"]
pub struct HCCPARAMS_SPEC;
impl crate::RegisterSpec for HCCPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccparams::R](R) reader structure"]
impl crate::Readable for HCCPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets hccparams to value 0"]
impl crate::Resettable for HCCPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
