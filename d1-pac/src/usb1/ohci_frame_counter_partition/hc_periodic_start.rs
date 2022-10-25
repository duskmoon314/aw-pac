#[doc = "Register `hc_periodic_start` reader"]
pub struct R(crate::R<HC_PERIODIC_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HC_PERIODIC_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HC_PERIODIC_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HC_PERIODIC_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hc_periodic_start` writer"]
pub struct W(crate::W<HC_PERIODIC_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HC_PERIODIC_START_SPEC>;
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
impl From<crate::W<HC_PERIODIC_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HC_PERIODIC_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `periodic_start` reader - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
pub type PERIODIC_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `periodic_start` writer - PeriodicStart\n\nAfter a hardware reset, this field is cleared. This is then set by HCD during the HC initialization. The value is calculated roughly as 10% off from. A typical value will be 0x2A3F (or 0x3e67). When reaches the value specified, processing of the periodic lists will have priority over Control/Bulk processing. HC will therefore start processing the Interrupt list after completing the current Control or Bulk transaction that is in progress."]
pub type PERIODIC_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HC_PERIODIC_START_SPEC, u16, u16, 14, O>;
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
    pub fn periodic_start(&mut self) -> PERIODIC_START_W<0> {
        PERIODIC_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OHCI Periodic Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc_periodic_start](index.html) module"]
pub struct HC_PERIODIC_START_SPEC;
impl crate::RegisterSpec for HC_PERIODIC_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hc_periodic_start::R](R) reader structure"]
impl crate::Readable for HC_PERIODIC_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hc_periodic_start::W](W) writer structure"]
impl crate::Writable for HC_PERIODIC_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hc_periodic_start to value 0"]
impl crate::Resettable for HC_PERIODIC_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
