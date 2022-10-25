#[doc = "Register `dmac_cur_dest%s` reader"]
pub struct R(crate::R<DMAC_CUR_DEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CUR_DEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CUR_DEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CUR_DEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Channel Current Destination Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_cur_dest](index.html) module"]
pub struct DMAC_CUR_DEST_SPEC;
impl crate::RegisterSpec for DMAC_CUR_DEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_cur_dest::R](R) reader structure"]
impl crate::Readable for DMAC_CUR_DEST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_cur_dest%s to value 0"]
impl crate::Resettable for DMAC_CUR_DEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
