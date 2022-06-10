#[doc = "Register `USBINTR` reader"]
pub struct R(crate::R<USBINTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBINTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBINTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBINTR` writer"]
pub struct W(crate::W<USBINTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINTR_SPEC>;
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
impl From<crate::W<USBINTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBINTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ON_ASYNC_ADVANCE_ENABLE` reader - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_ON_ASYNC_ADVANCE_ENABLE` writer - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 5>;
#[doc = "Field `HOST_SYSTEM_ERROR_ENABLE` reader - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
pub type HOST_SYSTEM_ERROR_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SYSTEM_ERROR_ENABLE` writer - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
pub type HOST_SYSTEM_ERROR_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 4>;
#[doc = "Field `FRAME_LIST_ROLLOVER_ENABLE` reader - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
pub type FRAME_LIST_ROLLOVER_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_LIST_ROLLOVER_ENABLE` writer - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
pub type FRAME_LIST_ROLLOVER_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 3>;
#[doc = "Field `PORT_CHANGE_INTERRUPT_ENABLE` reader - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
pub type PORT_CHANGE_INTERRUPT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PORT_CHANGE_INTERRUPT_ENABLE` writer - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
pub type PORT_CHANGE_INTERRUPT_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 2>;
#[doc = "Field `USB_ERROR_INTERRUPT_ENABLE` reader - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
pub type USB_ERROR_INTERRUPT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_ERROR_INTERRUPT_ENABLE` writer - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
pub type USB_ERROR_INTERRUPT_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 1>;
#[doc = "Field `USB_INTERRUPT_ENABLE` reader - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
pub type USB_INTERRUPT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_INTERRUPT_ENABLE` writer - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
pub type USB_INTERRUPT_ENABLE_W<'a> = crate::BitWriter<'a, u32, USBINTR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 5 - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
    #[inline(always)]
    pub fn interrupt_on_async_advance_enable(&self) -> INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R {
        INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
    #[inline(always)]
    pub fn host_system_error_enable(&self) -> HOST_SYSTEM_ERROR_ENABLE_R {
        HOST_SYSTEM_ERROR_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
    #[inline(always)]
    pub fn frame_list_rollover_enable(&self) -> FRAME_LIST_ROLLOVER_ENABLE_R {
        FRAME_LIST_ROLLOVER_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
    #[inline(always)]
    pub fn port_change_interrupt_enable(&self) -> PORT_CHANGE_INTERRUPT_ENABLE_R {
        PORT_CHANGE_INTERRUPT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
    #[inline(always)]
    pub fn usb_error_interrupt_enable(&self) -> USB_ERROR_INTERRUPT_ENABLE_R {
        USB_ERROR_INTERRUPT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
    #[inline(always)]
    pub fn usb_interrupt_enable(&self) -> USB_INTERRUPT_ENABLE_R {
        USB_INTERRUPT_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
    #[inline(always)]
    pub fn interrupt_on_async_advance_enable(&mut self) -> INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W {
        INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
    #[inline(always)]
    pub fn host_system_error_enable(&mut self) -> HOST_SYSTEM_ERROR_ENABLE_W {
        HOST_SYSTEM_ERROR_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
    #[inline(always)]
    pub fn frame_list_rollover_enable(&mut self) -> FRAME_LIST_ROLLOVER_ENABLE_W {
        FRAME_LIST_ROLLOVER_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
    #[inline(always)]
    pub fn port_change_interrupt_enable(&mut self) -> PORT_CHANGE_INTERRUPT_ENABLE_W {
        PORT_CHANGE_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
    #[inline(always)]
    pub fn usb_error_interrupt_enable(&mut self) -> USB_ERROR_INTERRUPT_ENABLE_W {
        USB_ERROR_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Bit 0 - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
    #[inline(always)]
    pub fn usb_interrupt_enable(&mut self) -> USB_INTERRUPT_ENABLE_W {
        USB_INTERRUPT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI USB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintr](index.html) module"]
pub struct USBINTR_SPEC;
impl crate::RegisterSpec for USBINTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbintr::R](R) reader structure"]
impl crate::Readable for USBINTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbintr::W](W) writer structure"]
impl crate::Writable for USBINTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for USBINTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
