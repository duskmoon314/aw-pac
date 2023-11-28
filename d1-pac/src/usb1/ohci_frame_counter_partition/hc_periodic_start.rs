#[doc = "Register `hc_periodic_start` reader"]
pub type R = crate::R<HC_PERIODIC_START_SPEC>;
#[doc = "Register `hc_periodic_start` writer"]
pub type W = crate::W<HC_PERIODIC_START_SPEC>;
#[doc = "Field `periodic_start` reader - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
pub type PERIODIC_START_R = crate::FieldReader<u16>;
#[doc = "Field `periodic_start` writer - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
pub type PERIODIC_START_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
    #[inline(always)]
    pub fn periodic_start(&self) -> PERIODIC_START_R {
        PERIODIC_START_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn periodic_start(&mut self) -> PERIODIC_START_W<HC_PERIODIC_START_SPEC> {
        PERIODIC_START_W::new(self, 0)
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
#[doc = "OHCI Periodic Start Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_periodic_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hc_periodic_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HC_PERIODIC_START_SPEC;
impl crate::RegisterSpec for HC_PERIODIC_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_periodic_start::R`](R) reader structure"]
impl crate::Readable for HC_PERIODIC_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hc_periodic_start::W`](W) writer structure"]
impl crate::Writable for HC_PERIODIC_START_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_periodic_start to value 0"]
impl crate::Resettable for HC_PERIODIC_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
