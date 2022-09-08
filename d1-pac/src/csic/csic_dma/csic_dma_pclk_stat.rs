#[doc = "Register `csic_dma_pclk_stat` reader"]
pub struct R(crate::R<CSIC_DMA_PCLK_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_DMA_PCLK_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_DMA_PCLK_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_DMA_PCLK_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_dma_pclk_stat` writer"]
pub struct W(crate::W<CSIC_DMA_PCLK_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_DMA_PCLK_STAT_SPEC>;
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
impl From<crate::W<CSIC_DMA_PCLK_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_DMA_PCLK_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pclk_cnt_line_min` reader - Indicates minimum pixel clock counter value for each line. Update at every vsync or framedone."]
pub type PCLK_CNT_LINE_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pclk_cnt_line_max` reader - Indicates maximum pixel clock counter value for each line. Update at every vsync or framedone."]
pub type PCLK_CNT_LINE_MAX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Indicates minimum pixel clock counter value for each line. Update at every vsync or framedone."]
    #[inline(always)]
    pub fn pclk_cnt_line_min(&self) -> PCLK_CNT_LINE_MIN_R {
        PCLK_CNT_LINE_MIN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Indicates maximum pixel clock counter value for each line. Update at every vsync or framedone."]
    #[inline(always)]
    pub fn pclk_cnt_line_max(&self) -> PCLK_CNT_LINE_MAX_R {
        PCLK_CNT_LINE_MAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
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
#[doc = "CSIC DMA PCLK Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_dma_pclk_stat](index.html) module"]
pub struct CSIC_DMA_PCLK_STAT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_PCLK_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_dma_pclk_stat::R](R) reader structure"]
impl crate::Readable for CSIC_DMA_PCLK_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_dma_pclk_stat::W](W) writer structure"]
impl crate::Writable for CSIC_DMA_PCLK_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets csic_dma_pclk_stat to value 0x7fff"]
impl crate::Resettable for CSIC_DMA_PCLK_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff
    }
}