#[doc = "Register `retite_pc0` reader"]
pub struct R(crate::R<RETITE_PC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETITE_PC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETITE_PC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETITE_PC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Retire PC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retite_pc0](index.html) module"]
pub struct RETITE_PC0_SPEC;
impl crate::RegisterSpec for RETITE_PC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retite_pc0::R](R) reader structure"]
impl crate::Readable for RETITE_PC0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets retite_pc0 to value 0"]
impl crate::Resettable for RETITE_PC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
