#[doc = "Register `rxdma_waddrl` reader"]
pub struct R(crate::R<RXDMA_WADDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_WADDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_WADDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_WADDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "UART RXDMA Write Address Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_waddrl](index.html) module"]
pub struct RXDMA_WADDRL_SPEC;
impl crate::RegisterSpec for RXDMA_WADDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_waddrl::R](R) reader structure"]
impl crate::Readable for RXDMA_WADDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxdma_waddrl to value 0"]
impl crate::Resettable for RXDMA_WADDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
