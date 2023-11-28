#[doc = "Register `csic_dma_fifo_thrs` reader"]
pub type R = crate::R<CSIC_DMA_FIFO_THRS_SPEC>;
#[doc = "Register `csic_dma_fifo_thrs` writer"]
pub type W = crate::W<CSIC_DMA_FIFO_THRS_SPEC>;
#[doc = "Field `fifo_thrs` reader - When FIFO occupied memory exceed the threshold, dram frequency can not change."]
pub type FIFO_THRS_R = crate::FieldReader<u16>;
#[doc = "Field `fifo_thrs` writer - When FIFO occupied memory exceed the threshold, dram frequency can not change."]
pub type FIFO_THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - When FIFO occupied memory exceed the threshold, dram frequency can not change."]
    #[inline(always)]
    pub fn fifo_thrs(&self) -> FIFO_THRS_R {
        FIFO_THRS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - When FIFO occupied memory exceed the threshold, dram frequency can not change."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_thrs(&mut self) -> FIFO_THRS_W<CSIC_DMA_FIFO_THRS_SPEC> {
        FIFO_THRS_W::new(self, 0)
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
#[doc = "CSIC DMA FIFO Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_fifo_thrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_fifo_thrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_FIFO_THRS_SPEC;
impl crate::RegisterSpec for CSIC_DMA_FIFO_THRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_fifo_thrs::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_FIFO_THRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_fifo_thrs::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_FIFO_THRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_fifo_thrs to value 0x0400"]
impl crate::Resettable for CSIC_DMA_FIFO_THRS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
