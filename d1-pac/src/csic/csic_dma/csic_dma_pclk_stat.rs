#[doc = "Register `csic_dma_pclk_stat` reader"]
pub type R = crate::R<CSIC_DMA_PCLK_STAT_SPEC>;
#[doc = "Register `csic_dma_pclk_stat` writer"]
pub type W = crate::W<CSIC_DMA_PCLK_STAT_SPEC>;
#[doc = "Field `pclk_cnt_line_min` reader - Indicates minimum pixel clock counter value for each line. Update at every vsync or framedone."]
pub type PCLK_CNT_LINE_MIN_R = crate::FieldReader<u16>;
#[doc = "Field `pclk_cnt_line_max` reader - Indicates maximum pixel clock counter value for each line. Update at every vsync or framedone."]
pub type PCLK_CNT_LINE_MAX_R = crate::FieldReader<u16>;
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
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CSIC DMA PCLK Statistic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_pclk_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_pclk_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_PCLK_STAT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_PCLK_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_pclk_stat::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_PCLK_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_pclk_stat::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_PCLK_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_pclk_stat to value 0x7fff"]
impl crate::Resettable for CSIC_DMA_PCLK_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff;
}
