#[doc = "Register `csic_dma_f0_bufa_result` reader"]
pub type R = crate::R<CSIC_DMA_F0_BUFA_RESULT_SPEC>;
#[doc = "Register `csic_dma_f0_bufa_result` writer"]
pub type W = crate::W<CSIC_DMA_F0_BUFA_RESULT_SPEC>;
#[doc = "Field `f0_bufa_result` reader - Indicate the final F0_BUFA address used for DMA or FBC after software configuration or hardware calculation from Buffer-A address register or buffer address fifo. Only used for debug."]
pub type F0_BUFA_RESULT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicate the final F0_BUFA address used for DMA or FBC after software configuration or hardware calculation from Buffer-A address register or buffer address fifo. Only used for debug."]
    #[inline(always)]
    pub fn f0_bufa_result(&self) -> F0_BUFA_RESULT_R {
        F0_BUFA_RESULT_R::new(self.bits)
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
#[doc = "CSIC DMA FIFO 0 Output Buffer-A Address Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f0_bufa_result::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f0_bufa_result::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_F0_BUFA_RESULT_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F0_BUFA_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_f0_bufa_result::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_F0_BUFA_RESULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_f0_bufa_result::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_F0_BUFA_RESULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f0_bufa_result to value 0"]
impl crate::Resettable for CSIC_DMA_F0_BUFA_RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
