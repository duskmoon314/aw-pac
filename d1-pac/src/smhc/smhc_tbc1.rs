#[doc = "Register `smhc_tbc1` reader"]
pub struct R(crate::R<SMHC_TBC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_TBC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_TBC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_TBC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Transferred Byte Count between Host Memory and Internal FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_tbc1](index.html) module"]
pub struct SMHC_TBC1_SPEC;
impl crate::RegisterSpec for SMHC_TBC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_tbc1::R](R) reader structure"]
impl crate::Readable for SMHC_TBC1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets smhc_tbc1 to value 0"]
impl crate::Resettable for SMHC_TBC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
