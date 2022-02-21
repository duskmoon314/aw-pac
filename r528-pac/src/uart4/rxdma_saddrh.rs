#[doc = "Register `RXDMA_SADDRH` reader"]
pub struct R(crate::R<RXDMA_SADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_SADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_SADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_SADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_SADDRH` writer"]
pub struct W(crate::W<RXDMA_SADDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_SADDRH_SPEC>;
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
impl From<crate::W<RXDMA_SADDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_SADDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `saddr` reader - RXDMA Buffer Start Address \\[33:32\\]"]
pub struct SADDR_R(crate::FieldReader<u8, u8>);
impl SADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `saddr` writer - RXDMA Buffer Start Address \\[33:32\\]"]
pub struct SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W {
        SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Buffer Start Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_saddrh](index.html) module"]
pub struct RXDMA_SADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_SADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_saddrh::R](R) reader structure"]
impl crate::Readable for RXDMA_SADDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_saddrh::W](W) writer structure"]
impl crate::Writable for RXDMA_SADDRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_SADDRH to value 0"]
impl crate::Resettable for RXDMA_SADDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
