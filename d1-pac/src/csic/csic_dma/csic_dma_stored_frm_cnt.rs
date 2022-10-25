#[doc = "Register `csic_dma_stored_frm_cnt` reader"]
pub struct R(crate::R<CSIC_DMA_STORED_FRM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_STORED_FRM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_STORED_FRM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_STORED_FRM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_stored_frm_cnt` writer"]
pub struct W(crate::W<CSIC_DMA_STORED_FRM_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_STORED_FRM_CNT_SPEC>;
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
impl From<crate::W<CSIC_DMA_STORED_FRM_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_STORED_FRM_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `csic_dma_stored_frm_cnt` reader - Indicates value of stored frames counter. When the counter value reaches CSIC_DMA_STORED_FRM_THRESHOLD, the counter is cleared to 0. Only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_STORED_FRM_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Indicates value of stored frames counter. When the counter value reaches CSIC_DMA_STORED_FRM_THRESHOLD, the counter is cleared to 0. Only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_stored_frm_cnt(&self) -> CSIC_DMA_STORED_FRM_CNT_R {
        CSIC_DMA_STORED_FRM_CNT_R::new((self.bits & 0xff) as u8)
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
#[doc = "CSIC DMA Stored Frame Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_stored_frm_cnt](index.html) module"]
pub struct CSIC_DMA_STORED_FRM_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_STORED_FRM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_stored_frm_cnt::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_STORED_FRM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_stored_frm_cnt::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_STORED_FRM_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_stored_frm_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_STORED_FRM_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
