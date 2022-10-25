#[doc = "Register `lradc_data` reader"]
pub struct R(crate::R<LRADC_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRADC_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRADC_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRADC_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `lradc_data` reader - LRADC Data"]
pub type LRADC_DATA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - LRADC Data"]
    #[inline(always)]
    pub fn lradc_data(&self) -> LRADC_DATA_R {
        LRADC_DATA_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LRADC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lradc_data](index.html) module"]
pub struct LRADC_DATA_SPEC;
impl crate::RegisterSpec for LRADC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lradc_data::R](R) reader structure"]
impl crate::Readable for LRADC_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets lradc_data to value 0"]
impl crate::Resettable for LRADC_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
