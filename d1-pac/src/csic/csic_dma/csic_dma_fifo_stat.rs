#[doc = "Register `csic_dma_fifo_stat` reader"]
pub type R = crate::R<CSIC_DMA_FIFO_STAT_SPEC>;
#[doc = "Register `csic_dma_fifo_stat` writer"]
pub type W = crate::W<CSIC_DMA_FIFO_STAT_SPEC>;
#[doc = "Field `fifo_frm_max` reader - Indicates the maximum depth of FIFO being occupied for whole frame. Update at every vsync or framedone."]
pub type FIFO_FRM_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `line` reader - Line Index Indicates the line index in current vsync."]
pub type LINE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Indicates the maximum depth of FIFO being occupied for whole frame. Update at every vsync or framedone."]
    #[inline(always)]
    pub fn fifo_frm_max(&self) -> FIFO_FRM_MAX_R {
        FIFO_FRM_MAX_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:29 - Line Index Indicates the line index in current vsync."]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
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
#[doc = "CSIC DMA FIFO Statistic Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_fifo_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_fifo_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_FIFO_STAT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FIFO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_fifo_stat::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_FIFO_STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_fifo_stat::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_FIFO_STAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_fifo_stat to value 0"]
impl crate::Resettable for CSIC_DMA_FIFO_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
