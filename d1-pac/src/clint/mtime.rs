#[doc = "Register `mtime` reader"]
pub struct R(crate::R<MTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "MTIME\n\nREF: opensbi\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime](index.html) module"]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtime::R](R) reader structure"]
impl crate::Readable for MTIME_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets mtime to value 0"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
