#[doc = "Register `csic_dma_vi_to_cnt_val` reader"]
pub struct R(crate::R<CSIC_DMA_VI_TO_CNT_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_VI_TO_CNT_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_VI_TO_CNT_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_vi_to_cnt_val` writer"]
pub struct W(crate::W<CSIC_DMA_VI_TO_CNT_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
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
impl From<crate::W<CSIC_DMA_VI_TO_CNT_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_VI_TO_CNT_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vi_to_cnt_val` reader - Video Input Timeout Counter Value\n\nIndicate the current value of Video Input Timeout Counter"]
pub type VI_TO_CNT_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Video Input Timeout Counter Value\n\nIndicate the current value of Video Input Timeout Counter"]
    #[inline(always)]
    pub fn vi_to_cnt_val(&self) -> VI_TO_CNT_VAL_R {
        VI_TO_CNT_VAL_R::new(self.bits)
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
#[doc = "CSIC DMA Video Input Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_vi_to_cnt_val](index.html) module"]
pub struct CSIC_DMA_VI_TO_CNT_VAL_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_vi_to_cnt_val::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_vi_to_cnt_val::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vi_to_cnt_val to value 0"]
impl crate::Resettable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
