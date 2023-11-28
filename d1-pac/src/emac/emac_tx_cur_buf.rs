#[doc = "Register `emac_tx_cur_buf` reader"]
pub type R = crate::R<EMAC_TX_CUR_BUF_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<EMAC_TX_CUR_BUF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "EMAC Current Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_tx_cur_buf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_TX_CUR_BUF_SPEC;
impl crate::RegisterSpec for EMAC_TX_CUR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_tx_cur_buf::R`](R) reader structure"]
impl crate::Readable for EMAC_TX_CUR_BUF_SPEC {}
#[doc = "`reset()` method sets emac_tx_cur_buf to value 0"]
impl crate::Resettable for EMAC_TX_CUR_BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
