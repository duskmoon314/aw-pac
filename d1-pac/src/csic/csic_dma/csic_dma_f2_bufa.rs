#[doc = "Register `csic_dma_f2_bufa` reader"]
pub type R = crate::R<CSIC_DMA_F2_BUFA_SPEC>;
#[doc = "Register `csic_dma_f2_bufa` writer"]
pub type W = crate::W<CSIC_DMA_F2_BUFA_SPEC>;
#[doc = "Field `f2_bufa` reader - FIFO 2 output buffer-A address."]
pub type F2_BUFA_R = crate::FieldReader<u32>;
#[doc = "Field `f2_bufa` writer - FIFO 2 output buffer-A address."]
pub type F2_BUFA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO 2 output buffer-A address."]
    #[inline(always)]
    pub fn f2_bufa(&self) -> F2_BUFA_R {
        F2_BUFA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO 2 output buffer-A address."]
    #[inline(always)]
    #[must_use]
    pub fn f2_bufa(&mut self) -> F2_BUFA_W<CSIC_DMA_F2_BUFA_SPEC> {
        F2_BUFA_W::new(self, 0)
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
#[doc = "CSIC DMA FIFO 2 Output Buffer-A Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_f2_bufa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_f2_bufa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_F2_BUFA_SPEC;
impl crate::RegisterSpec for CSIC_DMA_F2_BUFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_f2_bufa::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_F2_BUFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_f2_bufa::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_F2_BUFA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_f2_bufa to value 0"]
impl crate::Resettable for CSIC_DMA_F2_BUFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
