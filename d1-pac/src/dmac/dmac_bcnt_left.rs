#[doc = "Register `dmac_bcnt_left%s` reader"]
pub type R = crate::R<DMAC_BCNT_LEFT_SPEC>;
#[doc = "Field `dma_bcnt_left` reader - DMA Channel Byte Counter Left"]
pub type DMA_BCNT_LEFT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - DMA Channel Byte Counter Left"]
    #[inline(always)]
    pub fn dma_bcnt_left(&self) -> DMA_BCNT_LEFT_R {
        DMA_BCNT_LEFT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "DMAC Channel Byte Counter Left Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_bcnt_left::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_BCNT_LEFT_SPEC;
impl crate::RegisterSpec for DMAC_BCNT_LEFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_bcnt_left::R`](R) reader structure"]
impl crate::Readable for DMAC_BCNT_LEFT_SPEC {}
#[doc = "`reset()` method sets dmac_bcnt_left%s to value 0"]
impl crate::Resettable for DMAC_BCNT_LEFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
