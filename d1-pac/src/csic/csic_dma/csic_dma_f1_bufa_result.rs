#[doc = "Register `csic_dma_f1_bufa_result` reader"]
pub struct R(crate::R<CSIC_DMA_F1_BUFA_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_F1_BUFA_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_F1_BUFA_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_F1_BUFA_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_f1_bufa_result` writer"]
pub struct W(crate::W<CSIC_DMA_F1_BUFA_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_F1_BUFA_RESULT_SPEC>;
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
impl From<crate::W<CSIC_DMA_F1_BUFA_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_F1_BUFA_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `f1_bufa_result` reader - Indicate the final F1_BUFA address used for DMA or FBC after software configuration or hardware calculation from Buffer-A address register or buffer address fifo. Only used for debug."]
pub type F1_BUFA_RESULT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicate the final F1_BUFA address used for DMA or FBC after software configuration or hardware calculation from Buffer-A address register or buffer address fifo. Only used for debug."]
    #[inline(always)]
    pub fn f1_bufa_result(&self) -> F1_BUFA_RESULT_R {
        F1_BUFA_RESULT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_f1_bufa_result](index.html) module"]
pub struct CSIC_DMA_F1_BUFA_RESULT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F1_BUFA_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_f1_bufa_result::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_F1_BUFA_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_f1_bufa_result::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_F1_BUFA_RESULT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f1_bufa_result to value 0"]
impl crate::Resettable for CSIC_DMA_F1_BUFA_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
