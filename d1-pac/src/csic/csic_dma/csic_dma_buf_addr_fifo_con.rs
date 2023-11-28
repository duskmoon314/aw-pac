#[doc = "Register `csic_dma_buf_addr_fifo_con` reader"]
pub type R = crate::R<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
#[doc = "Register `csic_dma_buf_addr_fifo_con` writer"]
pub type W = crate::W<CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC>;
#[doc = "Field `csic_dma_buf_addr_fifo_content[0-2]` reader - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R = crate::FieldReader;
impl R {
    #[doc = "FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `csic_dma_buf_addr_fifo0_content` field"]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_content(&self, n: u8) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> (n * 8)) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo0_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo1_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - FIFO Content of address buffered in Buffer Address FIFO\\[i\\], only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo2_content(&self) -> CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R {
        CSIC_DMA_BUF_ADDR_FIFO_CONTENT_R::new(((self.bits >> 16) & 0x3f) as u8)
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
#[doc = "CSIC DMA BUF Address FIFO Content Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_addr_fifo_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_addr_fifo_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_buf_addr_fifo_con::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_buf_addr_fifo_con::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_buf_addr_fifo_con to value 0"]
impl crate::Resettable for CSIC_DMA_BUF_ADDR_FIFO_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
