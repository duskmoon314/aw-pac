#[doc = "Register `tp_data` reader"]
pub struct R(crate::R<TP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tp_data` reader - TP Data"]
pub type TP_DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - TP Data"]
    #[inline(always)]
    pub fn tp_data(&self) -> TP_DATA_R {
        TP_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "TP Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_data](index.html) module"]
pub struct TP_DATA_SPEC;
impl crate::RegisterSpec for TP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_data::R](R) reader structure"]
impl crate::Readable for TP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tp_data to value 0"]
impl crate::Resettable for TP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
