#[doc = "Register `csic_dma_f0_bufa` reader"]
pub struct R(crate::R<CSIC_DMA_F0_BUFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_F0_BUFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_F0_BUFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_F0_BUFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_f0_bufa` writer"]
pub struct W(crate::W<CSIC_DMA_F0_BUFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_F0_BUFA_SPEC>;
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
impl From<crate::W<CSIC_DMA_F0_BUFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_F0_BUFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `f0_bufa` reader - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate output address of overhead data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is disabled, these bits indicate FIFO 0 output buffer-A address in DMA mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is enabled, these bits indicate the output buffer address in LBC mode."]
pub type F0_BUFA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `f0_bufa` writer - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate output address of overhead data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is disabled, these bits indicate FIFO 0 output buffer-A address in DMA mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is enabled, these bits indicate the output buffer address in LBC mode."]
pub type F0_BUFA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_F0_BUFA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate output address of overhead data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is disabled, these bits indicate FIFO 0 output buffer-A address in DMA mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is enabled, these bits indicate the output buffer address in LBC mode."]
    #[inline(always)]
    pub fn f0_bufa(&self) -> F0_BUFA_R {
        F0_BUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate output address of overhead data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is disabled, these bits indicate FIFO 0 output buffer-A address in DMA mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, LBC_EN is enabled, these bits indicate the output buffer address in LBC mode."]
    #[inline(always)]
    #[must_use]
    pub fn f0_bufa(&mut self) -> F0_BUFA_W<0> {
        F0_BUFA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_f0_bufa](index.html) module"]
pub struct CSIC_DMA_F0_BUFA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F0_BUFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_f0_bufa::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_F0_BUFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_f0_bufa::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_F0_BUFA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f0_bufa to value 0"]
impl crate::Resettable for CSIC_DMA_F0_BUFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
