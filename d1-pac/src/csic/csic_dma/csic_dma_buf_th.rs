#[doc = "Register `csic_dma_buf_th` reader"]
pub type R = crate::R<CSIC_DMA_BUF_TH_SPEC>;
#[doc = "Register `csic_dma_buf_th` writer"]
pub type W = crate::W<CSIC_DMA_BUF_TH_SPEC>;
#[doc = "Field `csic_dma_buf_addr_fifo_threshold` reader - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `csic_dma_buf_addr_fifo_threshold` writer - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `csic_dma_stored_frm_threshold` reader - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_STORED_FRM_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `csic_dma_stored_frm_threshold` writer - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
pub type CSIC_DMA_STORED_FRM_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_buf_addr_fifo_threshold(&self) -> CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R {
        CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    pub fn csic_dma_stored_frm_threshold(&self) -> CSIC_DMA_STORED_FRM_THRESHOLD_R {
        CSIC_DMA_STORED_FRM_THRESHOLD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - when content in Buffer Address FIFO less than the threshold, an interrupt is set, only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    #[must_use]
    pub fn csic_dma_buf_addr_fifo_threshold(
        &mut self,
    ) -> CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W<CSIC_DMA_BUF_TH_SPEC> {
        CSIC_DMA_BUF_ADDR_FIFO_THRESHOLD_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - when stored frame counter value reaches the threshold , counter is cleared to 0 , only used in Buffer Addr FIFO Mode."]
    #[inline(always)]
    #[must_use]
    pub fn csic_dma_stored_frm_threshold(
        &mut self,
    ) -> CSIC_DMA_STORED_FRM_THRESHOLD_W<CSIC_DMA_BUF_TH_SPEC> {
        CSIC_DMA_STORED_FRM_THRESHOLD_W::new(self, 16)
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
#[doc = "CSIC DMA BUF Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_dma_buf_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_dma_buf_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_DMA_BUF_TH_SPEC;
impl crate::RegisterSpec for CSIC_DMA_BUF_TH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_dma_buf_th::R`](R) reader structure"]
impl crate::Readable for CSIC_DMA_BUF_TH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_dma_buf_th::W`](W) writer structure"]
impl crate::Writable for CSIC_DMA_BUF_TH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_dma_buf_th to value 0x0020_0000"]
impl crate::Resettable for CSIC_DMA_BUF_TH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
