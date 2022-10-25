#[doc = "Register `csic_dma_f2_bufa` reader"]
pub struct R(crate::R<CSIC_DMA_F2_BUFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_F2_BUFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_F2_BUFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_F2_BUFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_f2_bufa` writer"]
pub struct W(crate::W<CSIC_DMA_F2_BUFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_F2_BUFA_SPEC>;
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
impl From<crate::W<CSIC_DMA_F2_BUFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_F2_BUFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `f2_bufa` reader - FIFO 2 output buffer-A address."]
pub type F2_BUFA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `f2_bufa` writer - FIFO 2 output buffer-A address."]
pub type F2_BUFA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_F2_BUFA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - FIFO 2 output buffer-A address."]
    #[inline(always)]
    pub fn f2_bufa(&self) -> F2_BUFA_R {
        F2_BUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO 2 output buffer-A address."]
    #[inline(always)]
    #[must_use]
    pub fn f2_bufa(&mut self) -> F2_BUFA_W<0> {
        F2_BUFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_f2_bufa](index.html) module"]
pub struct CSIC_DMA_F2_BUFA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F2_BUFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_f2_bufa::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_F2_BUFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_f2_bufa::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_F2_BUFA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f2_bufa to value 0"]
impl crate::Resettable for CSIC_DMA_F2_BUFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
