#[doc = "Register `csic_dma_frm_clk_cnt` reader"]
pub type R = crate::R<CSIC_DMA_FRM_CLK_CNT_SPEC>;
#[doc = "Register `csic_dma_frm_clk_cnt` writer"]
pub type W = crate::W<CSIC_DMA_FRM_CLK_CNT_SPEC>;
#[doc = "Field `frm_clk_cnt` reader - Counter value between every frame. For instant hardware frame rate statics.\n\nThe internal counter is added by one every 12 MHz clock cycle. When frame done or vsync comes, the internal counter value is sampled to FRM_CLK_CNT, and cleared to 0."]
pub type FRM_CLK_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Counter value between every frame. For instant hardware frame rate statics.\n\nThe internal counter is added by one every 12 MHz clock cycle. When frame done or vsync comes, the internal counter value is sampled to FRM_CLK_CNT, and cleared to 0."]
    #[inline(always)]
    pub fn frm_clk_cnt(&self) -> FRM_CLK_CNT_R {
        FRM_CLK_CNT_R::new(self.bits & 0x00ff_ffff)
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
#[doc = "CSIC DMA Frame Clock Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_frm_clk_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_frm_clk_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_FRM_CLK_CNT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FRM_CLK_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_frm_clk_cnt::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_FRM_CLK_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_frm_clk_cnt::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_FRM_CLK_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_frm_clk_cnt to value 0"]
impl crate::Resettable for CSIC_DMA_FRM_CLK_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
