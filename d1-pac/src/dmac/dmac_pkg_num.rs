#[doc = "Register `dmac_pkg_num%s` reader"]
pub struct R(crate::R<DMAC_PKG_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PKG_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PKG_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PKG_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Package Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_pkg_num](index.html) module"]
pub struct DMAC_PKG_NUM_SPEC;
impl crate::RegisterSpec for DMAC_PKG_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_pkg_num::R](R) reader structure"]
impl crate::Readable for DMAC_PKG_NUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_pkg_num%s to value 0"]
impl crate::Resettable for DMAC_PKG_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
