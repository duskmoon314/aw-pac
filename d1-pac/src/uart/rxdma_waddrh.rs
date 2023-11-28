#[doc = "Register `rxdma_waddrh` reader"]
pub type R = crate::R<RXDMA_WADDRH_SPEC>;
#[doc = "Field `waddr` reader - RXDMA Current Write Address \\[33:32\\]"]
pub type WADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - RXDMA Current Write Address \\[33:32\\]"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new((self.bits & 3) as u8)
    }
}
#[doc = "UART RXDMA Write Address High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_waddrh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_WADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_WADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_waddrh::R`](R) reader structure"]
impl crate::Readable for RXDMA_WADDRH_SPEC {}
#[doc = "`reset()` method sets rxdma_waddrh to value 0"]
impl crate::Resettable for RXDMA_WADDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
