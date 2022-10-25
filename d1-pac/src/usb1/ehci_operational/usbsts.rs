#[doc = "Register `usbsts` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usbsts` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usbint` reader - USB Interrupt(USBINT) The Host Controller sets this bit to a one on the completion of a USB transaction, which results in the retirement of a Transfer Descriptor that had its IOC bit set. The Host Controller also sets this bit to 1 when a short packet is detected (actual number of bytes received was less than the expected number of bytes)"]
pub type USBINT_R = crate::BitReader<bool>;
#[doc = "Field `usbint` writer - USB Interrupt(USBINT) The Host Controller sets this bit to a one on the completion of a USB transaction, which results in the retirement of a Transfer Descriptor that had its IOC bit set. The Host Controller also sets this bit to 1 when a short packet is detected (actual number of bytes received was less than the expected number of bytes)"]
pub type USBINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `usberrint` reader - USB Error Interrupt(USBERRINT)\n\nThe Host Controller sets this bit to 1 when completion of USB transaction results in an error condition(e.g. error counter underflow).If the TD on which the error interrupt occurred also had its IOC bit set, both. This bit and USBINT bit are set."]
pub type USBERRINT_R = crate::BitReader<bool>;
#[doc = "Field `usberrint` writer - USB Error Interrupt(USBERRINT)\n\nThe Host Controller sets this bit to 1 when completion of USB transaction results in an error condition(e.g. error counter underflow).If the TD on which the error interrupt occurred also had its IOC bit set, both. This bit and USBINT bit are set."]
pub type USBERRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `port_change_detect` reader - Port Change Detect\n\nThe Host Controller sets this bit to a one when any port for which the Port Owner bit is set to zero has a change bit transition from a zero to a one or a Force Port Resume bit transition from a zero to a one as a result of a J-K transition detected on a suspended port. This bit will also be set as a result of the Connect Status Chang being set to a one after system software has relinquished ownership of a connected port by writing a one to a port's Port Owner bit."]
pub type PORT_CHANGE_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `port_change_detect` writer - Port Change Detect\n\nThe Host Controller sets this bit to a one when any port for which the Port Owner bit is set to zero has a change bit transition from a zero to a one or a Force Port Resume bit transition from a zero to a one as a result of a J-K transition detected on a suspended port. This bit will also be set as a result of the Connect Status Chang being set to a one after system software has relinquished ownership of a connected port by writing a one to a port's Port Owner bit."]
pub type PORT_CHANGE_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `frame_list_rollover` reader - Frame List Rollover\n\nThe Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX \\[12\\] toggles."]
pub type FRAME_LIST_ROLLOVER_R = crate::BitReader<bool>;
#[doc = "Field `frame_list_rollover` writer - Frame List Rollover\n\nThe Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX \\[12\\] toggles."]
pub type FRAME_LIST_ROLLOVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `host_system_error` reader - Host System Error\n\nThe Host Controller set this bit to 1 when a serious error occurs during a host system access involving the Host Controller module. When this error occurs, the Host Controller clears the Run/Stop bit in the Command register to prevent further execution of the scheduled TDs."]
pub type HOST_SYSTEM_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `host_system_error` writer - Host System Error\n\nThe Host Controller set this bit to 1 when a serious error occurs during a host system access involving the Host Controller module. When this error occurs, the Host Controller clears the Run/Stop bit in the Command register to prevent further execution of the scheduled TDs."]
pub type HOST_SYSTEM_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `interrupt_on_async_advance` reader - Interrupt on Async Advance\n\nSystem software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_R = crate::BitReader<bool>;
#[doc = "Field `interrupt_on_async_advance` writer - Interrupt on Async Advance\n\nSystem software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `hc_halted` reader - HC Halted\n\nThis bit is a zero whenever the Run/Stop bit is a one. The Host Controller Sets this bit to one after it has stopped executing as a result of the Run/Stop bit being set to 0, either by software or by the Host Controller Hardware (e.g. internal error).\n\nThe default value is '1'"]
pub type HC_HALTED_R = crate::BitReader<bool>;
#[doc = "Field `reclamation` reader - Reclamation\n\nThis is a read-only status bit, which is used to detect an empty asynchronous schedule."]
pub type RECLAMATION_R = crate::BitReader<bool>;
#[doc = "Field `periodic_schedule_status` reader - Periodic Schedule Status\n\nThe bit reports the current real status of the Periodic Schedule. If this bit is a zero then the status of the Periodic Schedule is disabled. If this bit is a one then the status of the Periodic Schedule is enabled. The Host Controller is not required to disable or enable the Periodic Schedule when software transitions the bit in the USBCMD register. When this bit and the bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0)."]
pub type PERIODIC_SCHEDULE_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `asynchronous_schedule_status` reader - Asynchronous Schedule Status\n\nThe bit reports the current real status of Asynchronous Schedule. If this bit is a zero then the status of the Asynchronous Schedule is disabled. If this bit is a one then the status of the Asynchronous Schedule is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0)."]
pub type ASYNCHRONOUS_SCHEDULE_STATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USB Interrupt(USBINT) The Host Controller sets this bit to a one on the completion of a USB transaction, which results in the retirement of a Transfer Descriptor that had its IOC bit set. The Host Controller also sets this bit to 1 when a short packet is detected (actual number of bytes received was less than the expected number of bytes)"]
    #[inline(always)]
    pub fn usbint(&self) -> USBINT_R {
        USBINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt(USBERRINT)\n\nThe Host Controller sets this bit to 1 when completion of USB transaction results in an error condition(e.g. error counter underflow).If the TD on which the error interrupt occurred also had its IOC bit set, both. This bit and USBINT bit are set."]
    #[inline(always)]
    pub fn usberrint(&self) -> USBERRINT_R {
        USBERRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect\n\nThe Host Controller sets this bit to a one when any port for which the Port Owner bit is set to zero has a change bit transition from a zero to a one or a Force Port Resume bit transition from a zero to a one as a result of a J-K transition detected on a suspended port. This bit will also be set as a result of the Connect Status Chang being set to a one after system software has relinquished ownership of a connected port by writing a one to a port's Port Owner bit."]
    #[inline(always)]
    pub fn port_change_detect(&self) -> PORT_CHANGE_DETECT_R {
        PORT_CHANGE_DETECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover\n\nThe Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX \\[12\\] toggles."]
    #[inline(always)]
    pub fn frame_list_rollover(&self) -> FRAME_LIST_ROLLOVER_R {
        FRAME_LIST_ROLLOVER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error\n\nThe Host Controller set this bit to 1 when a serious error occurs during a host system access involving the Host Controller module. When this error occurs, the Host Controller clears the Run/Stop bit in the Command register to prevent further execution of the scheduled TDs."]
    #[inline(always)]
    pub fn host_system_error(&self) -> HOST_SYSTEM_ERROR_R {
        HOST_SYSTEM_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance\n\nSystem software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
    #[inline(always)]
    pub fn interrupt_on_async_advance(&self) -> INTERRUPT_ON_ASYNC_ADVANCE_R {
        INTERRUPT_ON_ASYNC_ADVANCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - HC Halted\n\nThis bit is a zero whenever the Run/Stop bit is a one. The Host Controller Sets this bit to one after it has stopped executing as a result of the Run/Stop bit being set to 0, either by software or by the Host Controller Hardware (e.g. internal error).\n\nThe default value is '1'"]
    #[inline(always)]
    pub fn hc_halted(&self) -> HC_HALTED_R {
        HC_HALTED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reclamation\n\nThis is a read-only status bit, which is used to detect an empty asynchronous schedule."]
    #[inline(always)]
    pub fn reclamation(&self) -> RECLAMATION_R {
        RECLAMATION_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status\n\nThe bit reports the current real status of the Periodic Schedule. If this bit is a zero then the status of the Periodic Schedule is disabled. If this bit is a one then the status of the Periodic Schedule is enabled. The Host Controller is not required to disable or enable the Periodic Schedule when software transitions the bit in the USBCMD register. When this bit and the bit are the same value, the Periodic Schedule is either enabled (1) or disabled (0)."]
    #[inline(always)]
    pub fn periodic_schedule_status(&self) -> PERIODIC_SCHEDULE_STATUS_R {
        PERIODIC_SCHEDULE_STATUS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status\n\nThe bit reports the current real status of Asynchronous Schedule. If this bit is a zero then the status of the Asynchronous Schedule is disabled. If this bit is a one then the status of the Asynchronous Schedule is enabled. The Host Controller is not required to immediately disable or enable the Asynchronous Schedule when software transitions the Asynchronous Schedule Enable bit in the USBCMD register. When this bit and the Asynchronous Schedule Enable bit are the same value, the Asynchronous Schedule is either enabled (1) or disabled (0)."]
    #[inline(always)]
    pub fn asynchronous_schedule_status(&self) -> ASYNCHRONOUS_SCHEDULE_STATUS_R {
        ASYNCHRONOUS_SCHEDULE_STATUS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt(USBINT) The Host Controller sets this bit to a one on the completion of a USB transaction, which results in the retirement of a Transfer Descriptor that had its IOC bit set. The Host Controller also sets this bit to 1 when a short packet is detected (actual number of bytes received was less than the expected number of bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn usbint(&mut self) -> USBINT_W<0> {
        USBINT_W::new(self)
    }
    #[doc = "Bit 1 - USB Error Interrupt(USBERRINT)\n\nThe Host Controller sets this bit to 1 when completion of USB transaction results in an error condition(e.g. error counter underflow).If the TD on which the error interrupt occurred also had its IOC bit set, both. This bit and USBINT bit are set."]
    #[inline(always)]
    #[must_use]
    pub fn usberrint(&mut self) -> USBERRINT_W<1> {
        USBERRINT_W::new(self)
    }
    #[doc = "Bit 2 - Port Change Detect\n\nThe Host Controller sets this bit to a one when any port for which the Port Owner bit is set to zero has a change bit transition from a zero to a one or a Force Port Resume bit transition from a zero to a one as a result of a J-K transition detected on a suspended port. This bit will also be set as a result of the Connect Status Chang being set to a one after system software has relinquished ownership of a connected port by writing a one to a port's Port Owner bit."]
    #[inline(always)]
    #[must_use]
    pub fn port_change_detect(&mut self) -> PORT_CHANGE_DETECT_W<2> {
        PORT_CHANGE_DETECT_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover\n\nThe Host Controller sets this bit to a one when the Frame List Index rolls over from its maximum value to zero. The exact value at which the rollover occurs depends on the frame list size. For example, if the frame list size is 1024, the Frame Index Register rolls over every time FRINDEX \\[13\\] toggles. Similarly, if the size is 512, the Host Controller sets this bit to a one every time FRINDEX \\[12\\] toggles."]
    #[inline(always)]
    #[must_use]
    pub fn frame_list_rollover(&mut self) -> FRAME_LIST_ROLLOVER_W<3> {
        FRAME_LIST_ROLLOVER_W::new(self)
    }
    #[doc = "Bit 4 - Host System Error\n\nThe Host Controller set this bit to 1 when a serious error occurs during a host system access involving the Host Controller module. When this error occurs, the Host Controller clears the Run/Stop bit in the Command register to prevent further execution of the scheduled TDs."]
    #[inline(always)]
    #[must_use]
    pub fn host_system_error(&mut self) -> HOST_SYSTEM_ERROR_W<4> {
        HOST_SYSTEM_ERROR_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance\n\nSystem software can force the host controller to issue an interrupt the next time the host controller advances the asynchronous schedule by writing a one to the Interrupt on Async Advance Doorbell bit in the USBCMD register. This status bit indicates the assertion of that interrupt source."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_async_advance(&mut self) -> INTERRUPT_ON_ASYNC_ADVANCE_W<5> {
        INTERRUPT_ON_ASYNC_ADVANCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI USB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usbsts to value 0x1000"]
impl crate::Resettable for USBSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
