#[doc = "Register `usbintr` reader"]
pub type R = crate::R<USBINTR_SPEC>;
#[doc = "Register `usbintr` writer"]
pub type W = crate::W<USBINTR_SPEC>;
#[doc = "Field `usb_interrupt_enable` reader - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
pub type USB_INTERRUPT_ENABLE_R = crate::BitReader;
#[doc = "Field `usb_interrupt_enable` writer - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
pub type USB_INTERRUPT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb_error_interrupt_enable` reader - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
pub type USB_ERROR_INTERRUPT_ENABLE_R = crate::BitReader;
#[doc = "Field `usb_error_interrupt_enable` writer - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
pub type USB_ERROR_INTERRUPT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `port_change_interrupt_enable` reader - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
pub type PORT_CHANGE_INTERRUPT_ENABLE_R = crate::BitReader;
#[doc = "Field `port_change_interrupt_enable` writer - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
pub type PORT_CHANGE_INTERRUPT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frame_list_rollover_enable` reader - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
pub type FRAME_LIST_ROLLOVER_ENABLE_R = crate::BitReader;
#[doc = "Field `frame_list_rollover_enable` writer - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
pub type FRAME_LIST_ROLLOVER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `host_system_error_enable` reader - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
pub type HOST_SYSTEM_ERROR_ENABLE_R = crate::BitReader;
#[doc = "Field `host_system_error_enable` writer - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
pub type HOST_SYSTEM_ERROR_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `interrupt_on_async_advance_enable` reader - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_on_async_advance_enable` writer - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
pub type INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
    #[inline(always)]
    pub fn usb_interrupt_enable(&self) -> USB_INTERRUPT_ENABLE_R {
        USB_INTERRUPT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
    #[inline(always)]
    pub fn usb_error_interrupt_enable(&self) -> USB_ERROR_INTERRUPT_ENABLE_R {
        USB_ERROR_INTERRUPT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
    #[inline(always)]
    pub fn port_change_interrupt_enable(&self) -> PORT_CHANGE_INTERRUPT_ENABLE_R {
        PORT_CHANGE_INTERRUPT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
    #[inline(always)]
    pub fn frame_list_rollover_enable(&self) -> FRAME_LIST_ROLLOVER_ENABLE_R {
        FRAME_LIST_ROLLOVER_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
    #[inline(always)]
    pub fn host_system_error_enable(&self) -> HOST_SYSTEM_ERROR_ENABLE_R {
        HOST_SYSTEM_ERROR_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
    #[inline(always)]
    pub fn interrupt_on_async_advance_enable(&self) -> INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R {
        INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt Enable\n\nWhen this bit is 1, and the USBINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBINT bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb_interrupt_enable(&mut self) -> USB_INTERRUPT_ENABLE_W<USBINTR_SPEC> {
        USB_INTERRUPT_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable\n\nWhen this bit is 1, and the USBERRINT bit in the USBSTS register is 1,the host controller will issue an interrupt at the next interrupt threshold.\n\nThe interrupt is acknowledged by software clearing the USBERRINT bit."]
    #[inline(always)]
    #[must_use]
    pub fn usb_error_interrupt_enable(&mut self) -> USB_ERROR_INTERRUPT_ENABLE_W<USBINTR_SPEC> {
        USB_ERROR_INTERRUPT_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable\n\nWhen this bit is 1, and the Port Chang Detect bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Port Chang Detect bit."]
    #[inline(always)]
    #[must_use]
    pub fn port_change_interrupt_enable(&mut self) -> PORT_CHANGE_INTERRUPT_ENABLE_W<USBINTR_SPEC> {
        PORT_CHANGE_INTERRUPT_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable\n\nWhen this bit is 1, and the Frame List Rollover bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Frame List Rollover bit."]
    #[inline(always)]
    #[must_use]
    pub fn frame_list_rollover_enable(&mut self) -> FRAME_LIST_ROLLOVER_ENABLE_W<USBINTR_SPEC> {
        FRAME_LIST_ROLLOVER_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error Enable\n\n When this bit is 1, and the Host System Error Status bit in the USBSTS register is 1, the host controller will issue an interrupt. The interrupt is acknowledged by software clearing the Host System Error bit."]
    #[inline(always)]
    #[must_use]
    pub fn host_system_error_enable(&mut self) -> HOST_SYSTEM_ERROR_ENABLE_W<USBINTR_SPEC> {
        HOST_SYSTEM_ERROR_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable\n\nWhen this bit is 1, and the Interrupt on Async Advance bit in the USBSTS register is 1, the host controller will issue an interrupt at the next interrupt threshold. The interrupt is acknowledged by software clearing the Interrupt on Async Advance bit."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_on_async_advance_enable(
        &mut self,
    ) -> INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W<USBINTR_SPEC> {
        INTERRUPT_ON_ASYNC_ADVANCE_ENABLE_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EHCI USB Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBINTR_SPEC;
impl crate::RegisterSpec for USBINTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintr::R`](R) reader structure"]
impl crate::Readable for USBINTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbintr::W`](W) writer structure"]
impl crate::Writable for USBINTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usbintr to value 0"]
impl crate::Resettable for USBINTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
