#[doc = "Register `dmac_pkg_num_reg%s` reader"]
pub struct R(crate::R<DMAC_PKG_NUM_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PKG_NUM_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PKG_NUM_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PKG_NUM_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DMAC Package Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_pkg_num_reg](index.html) module"]
pub struct DMAC_PKG_NUM_REG_SPEC;
impl crate::RegisterSpec for DMAC_PKG_NUM_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_pkg_num_reg::R](R) reader structure"]
impl crate::Readable for DMAC_PKG_NUM_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_pkg_num_reg%s to value 0"]
impl crate::Resettable for DMAC_PKG_NUM_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
