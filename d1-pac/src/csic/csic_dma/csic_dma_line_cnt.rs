#[doc = "Register `csic_dma_line_cnt` reader"]
pub struct R(crate::R<CSIC_DMA_LINE_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_LINE_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_LINE_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_LINE_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_line_cnt` writer"]
pub struct W(crate::W<CSIC_DMA_LINE_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_LINE_CNT_SPEC>;
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
impl From<crate::W<CSIC_DMA_LINE_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_LINE_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `line_cnt_num` reader - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
pub type LINE_CNT_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `line_cnt_num` writer - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
pub type LINE_CNT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_DMA_LINE_CNT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
    #[inline(always)]
    pub fn line_cnt_num(&self) -> LINE_CNT_NUM_R {
        LINE_CNT_NUM_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - The LINE_CNT_NUM value is set by user,when internal line counter reach the set value, the LC_PD will be set."]
    #[inline(always)]
    #[must_use]
    pub fn line_cnt_num(&mut self) -> LINE_CNT_NUM_W<0> {
        LINE_CNT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC DMA LINE Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_line_cnt](index.html) module"]
pub struct CSIC_DMA_LINE_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_LINE_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_line_cnt::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_LINE_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_line_cnt::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_LINE_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_line_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_LINE_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
