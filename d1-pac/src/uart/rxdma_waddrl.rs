#[doc = "Register `rxdma_waddrl` reader"]
pub type R = crate::R<RXDMA_WADDRL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXDMA_WADDRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "UART RXDMA Write Address Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_waddrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_WADDRL_SPEC;
impl crate::RegisterSpec for RXDMA_WADDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_waddrl::R`](R) reader structure"]
impl crate::Readable for RXDMA_WADDRL_SPEC {}
#[doc = "`reset()` method sets rxdma_waddrl to value 0"]
impl crate::Resettable for RXDMA_WADDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
