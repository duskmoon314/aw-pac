#[doc = "Register `rxdma_saddrh` reader"]
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
#[doc = "Register `rxdma_saddrh` writer"]
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
pub type SADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `saddr` writer - RXDMA Buffer Start Address \\[33:32\\]"]
pub type SADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXDMA_SADDRH_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXDMA Buffer Start Address \\[33:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SADDR_W<0> {
        SADDR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_saddrh to value 0"]
impl crate::Resettable for RXDMA_SADDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
