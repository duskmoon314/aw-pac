#[doc = "Register `emac_tx_cur_desc` reader"]
pub struct R(crate::R<EMAC_TX_CUR_DESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_TX_CUR_DESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_TX_CUR_DESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_TX_CUR_DESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "EMAC Current Transmit Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_tx_cur_desc](index.html) module"]
pub struct EMAC_TX_CUR_DESC_SPEC;
impl crate::RegisterSpec for EMAC_TX_CUR_DESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_tx_cur_desc::R](R) reader structure"]
impl crate::Readable for EMAC_TX_CUR_DESC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets emac_tx_cur_desc to value 0"]
impl crate::Resettable for EMAC_TX_CUR_DESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
