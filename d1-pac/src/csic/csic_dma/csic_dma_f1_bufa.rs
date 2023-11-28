#[doc = "Register `csic_dma_f1_bufa` reader"]
pub type R = crate::R<CSIC_DMA_F1_BUFA_SPEC>;
#[doc = "Register `csic_dma_f1_bufa` writer"]
pub type W = crate::W<CSIC_DMA_F1_BUFA_SPEC>;
#[doc = "Field `f1_bufa` reader - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate the output address of compressed data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, these bits indicate the FIFO 1 output buffer-A address in DMA mode."]
pub type F1_BUFA_R = crate::FieldReader<u32>;
#[doc = "Field `f1_bufa` writer - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate the output address of compressed data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, these bits indicate the FIFO 1 output buffer-A address in DMA mode."]
pub type F1_BUFA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate the output address of compressed data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, these bits indicate the FIFO 1 output buffer-A address in DMA mode."]
    #[inline(always)]
    pub fn f1_bufa(&self) -> F1_BUFA_R {
        F1_BUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When BK_TOP_EN is enabled, FBC_EN is enabled, DMA_EN is disabled, these bits indicate the output address of compressed data in FBC mode.\n\nWhen BK_TOP_EN is enabled, FBC_EN is disabled, DMA_EN is enabled, these bits indicate the FIFO 1 output buffer-A address in DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn f1_bufa(&mut self) -> F1_BUFA_W<CSIC_DMA_F1_BUFA_SPEC> {
        F1_BUFA_W::new(self, 0)
    }
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
#[doc = "CSIC DMA FIFO 1 Output Buffer-A Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f1_bufa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f1_bufa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_F1_BUFA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F1_BUFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_f1_bufa::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_F1_BUFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_f1_bufa::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_F1_BUFA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f1_bufa to value 0"]
impl crate::Resettable for CSIC_DMA_F1_BUFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
