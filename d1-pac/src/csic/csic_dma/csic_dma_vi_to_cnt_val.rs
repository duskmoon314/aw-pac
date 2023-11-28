#[doc = "Register `csic_dma_vi_to_cnt_val` reader"]
pub type R = crate::R<CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
#[doc = "Register `csic_dma_vi_to_cnt_val` writer"]
pub type W = crate::W<CSIC_DMA_VI_TO_CNT_VAL_SPEC>;
#[doc = "Field `vi_to_cnt_val` reader - Video Input Timeout Counter Value\n\nIndicate the current value of Video Input Timeout Counter"]
pub type VI_TO_CNT_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Video Input Timeout Counter Value\n\nIndicate the current value of Video Input Timeout Counter"]
    #[inline(always)]
    pub fn vi_to_cnt_val(&self) -> VI_TO_CNT_VAL_R {
        VI_TO_CNT_VAL_R::new(self.bits)
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
#[doc = "CSIC DMA Video Input Timeout Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_vi_to_cnt_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_vi_to_cnt_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_VI_TO_CNT_VAL_SPEC;
impl crate::RegisterSpec for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_vi_to_cnt_val::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_vi_to_cnt_val::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_vi_to_cnt_val to value 0"]
impl crate::Resettable for CSIC_DMA_VI_TO_CNT_VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
