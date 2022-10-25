#[doc = "Register `ths_data` reader"]
pub struct R(crate::R<THS_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ths_data` reader - Temperature measurement data of sensor"]
pub type THS_DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Temperature measurement data of sensor"]
    #[inline(always)]
    pub fn ths_data(&self) -> THS_DATA_R {
        THS_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "THS Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_data](index.html) module"]
pub struct THS_DATA_SPEC;
impl crate::RegisterSpec for THS_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_data::R](R) reader structure"]
impl crate::Readable for THS_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ths_data to value 0"]
impl crate::Resettable for THS_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
