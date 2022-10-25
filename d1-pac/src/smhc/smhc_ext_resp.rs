#[doc = "Register `smhc_ext_resp` reader"]
pub struct R(crate::R<SMHC_EXT_RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_EXT_RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_EXT_RESP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_EXT_RESP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Extended Response Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ext_resp](index.html) module"]
pub struct SMHC_EXT_RESP_SPEC;
impl crate::RegisterSpec for SMHC_EXT_RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ext_resp::R](R) reader structure"]
impl crate::Readable for SMHC_EXT_RESP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_ext_resp to value 0"]
impl crate::Resettable for SMHC_EXT_RESP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
