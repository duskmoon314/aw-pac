#[doc = "Register `rescal_status` reader"]
pub struct R(crate::R<RESCAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESCAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESCAL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESCAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `res_cal_do` reader - RESCAL Calibration Results Output"]
pub type RES_CAL_DO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cout` reader - Calibration Circuits Analog COmpare Output"]
pub type COUT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - RESCAL Calibration Results Output"]
    #[inline(always)]
    pub fn res_cal_do(&self) -> RES_CAL_DO_R {
        RES_CAL_DO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Calibration Circuits Analog COmpare Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Resistor Calibration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rescal_status](index.html) module"]
pub struct RESCAL_STATUS_SPEC;
impl crate::RegisterSpec for RESCAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rescal_status::R](R) reader structure"]
impl crate::Readable for RESCAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rescal_status to value 0"]
impl crate::Resettable for RESCAL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
