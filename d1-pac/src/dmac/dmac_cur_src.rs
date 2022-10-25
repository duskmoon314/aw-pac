#[doc = "Register `dmac_cur_src%s` reader"]
pub struct R(crate::R<DMAC_CUR_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CUR_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CUR_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CUR_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Channel Current Source Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_cur_src](index.html) module"]
pub struct DMAC_CUR_SRC_SPEC;
impl crate::RegisterSpec for DMAC_CUR_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_cur_src::R](R) reader structure"]
impl crate::Readable for DMAC_CUR_SRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_cur_src%s to value 0"]
impl crate::Resettable for DMAC_CUR_SRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
