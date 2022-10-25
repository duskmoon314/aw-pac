#[doc = "Register `smhc_tbc0` reader"]
pub struct R(crate::R<SMHC_TBC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_TBC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_TBC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_TBC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transferred Byte Count between Controller and Card\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_tbc0](index.html) module"]
pub struct SMHC_TBC0_SPEC;
impl crate::RegisterSpec for SMHC_TBC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_tbc0::R](R) reader structure"]
impl crate::Readable for SMHC_TBC0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_tbc0 to value 0"]
impl crate::Resettable for SMHC_TBC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
