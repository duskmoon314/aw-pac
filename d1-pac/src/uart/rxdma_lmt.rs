#[doc = "Register `rxdma_lmt` reader"]
pub struct R(crate::R<RXDMA_LMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_LMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_LMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_LMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxdma_lmt` writer"]
pub struct W(crate::W<RXDMA_LMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_LMT_SPEC>;
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
impl From<crate::W<RXDMA_LMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_LMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `limit_size` reader - "]
pub type LIMIT_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `limit_size` writer - "]
pub type LIMIT_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXDMA_LMT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn limit_size(&self) -> LIMIT_SIZE_R {
        LIMIT_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn limit_size(&mut self) -> LIMIT_SIZE_W<0> {
        LIMIT_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_lmt](index.html) module"]
pub struct RXDMA_LMT_SPEC;
impl crate::RegisterSpec for RXDMA_LMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_lmt::R](R) reader structure"]
impl crate::Readable for RXDMA_LMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_lmt::W](W) writer structure"]
impl crate::Writable for RXDMA_LMT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxdma_lmt to value 0"]
impl crate::Resettable for RXDMA_LMT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
