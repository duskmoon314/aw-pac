#[doc = "Register `csic_dma_buf_addr_fifo%s_entry` reader"]
pub type R = crate::R<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
#[doc = "Register `csic_dma_buf_addr_fifo%s_entry` writer"]
pub type W = crate::W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC>;
#[doc = "Field `csic_dma_buf_addr_fifo_entry` reader - FIFO Entry of Buffer Address FIFO\\[i\\] for input frames to be stored, only used in Buffer Addr FIFO Mode"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R = crate::FieldReader<u32>;
#[doc = "Field `csic_dma_buf_addr_fifo_entry` writer - FIFO Entry of Buffer Address FIFO\\[i\\] for input frames to be stored, only used in Buffer Addr FIFO Mode"]
pub type CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Entry of Buffer Address FIFO\\[i\\] for input frames to be stored, only used in Buffer Addr FIFO Mode"]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_entry(&self) -> CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R {
        CSIC_DMA_BUF_ADDR_FIFO_ENTRY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFO Entry of Buffer Address FIFO\\[i\\] for input frames to be stored, only used in Buffer Addr FIFO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn csic_dma_buf_addr_fifo_entry(
        &mut self,
    ) -> CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W<CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC> {
        CSIC_DMA_BUF_ADDR_FIFO_ENTRY_W::new(self, 0)
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
#[doc = "CSIC DMA BUF Address FIFO\\[i\\] Entry Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_addr_fifo_entry::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_addr_fifo_entry::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_buf_addr_fifo_entry::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_buf_addr_fifo_entry::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_buf_addr_fifo%s_entry to value 0"]
impl crate::Resettable for CSIC_DMA_BUF_ADDR_FIFO_ENTRY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
