#[doc = "Register `csic_dma_vi_to_th1` reader"]
pub struct R(crate::R<CSIC_DMA_VI_TO_TH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_VI_TO_TH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_VI_TO_TH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_VI_TO_TH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_vi_to_th1` writer"]
pub struct W(crate::W<CSIC_DMA_VI_TO_TH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_VI_TO_TH1_SPEC>;
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
impl From<crate::W<CSIC_DMA_VI_TO_TH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_VI_TO_TH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vi_to_th1` reader - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
pub type VI_TO_TH1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `vi_to_th1` writer - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
pub type VI_TO_TH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_VI_TO_TH1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
    #[inline(always)]
    pub fn vi_to_th1(&self) -> VI_TO_TH1_R {
        VI_TO_TH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Video Input Timeout Threshold1\n\nSet VIDEO_INPUT_TO_INT_PD when VI Counter reaches TH1 after getting the first frame has been input, the Time Unit is a 12M clock period."]
    #[inline(always)]
    #[must_use]
    pub fn vi_to_th1(&mut self) -> VI_TO_TH1_W<0> {
        VI_TO_TH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA Video Input Timeout Threshold1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_vi_to_th1](index.html) module"]
pub struct CSIC_DMA_VI_TO_TH1_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VI_TO_TH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_vi_to_th1::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_VI_TO_TH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_vi_to_th1::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_VI_TO_TH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vi_to_th1 to value 0"]
impl crate::Resettable for CSIC_DMA_VI_TO_TH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
