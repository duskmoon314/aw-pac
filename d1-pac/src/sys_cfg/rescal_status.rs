#[doc = "Register `RESCAL_STATUS` reader"]
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
#[doc = "Field `COUT` reader - Calibration Circuits Analog COmpare Output"]
pub struct COUT_R(crate::FieldReader<bool, bool>);
impl COUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RES_CAL_DO` reader - RESCAL Calibration Results Output"]
pub struct RES_CAL_DO_R(crate::FieldReader<u8, u8>);
impl RES_CAL_DO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RES_CAL_DO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_CAL_DO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8 - Calibration Circuits Analog COmpare Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:5 - RESCAL Calibration Results Output"]
    #[inline(always)]
    pub fn res_cal_do(&self) -> RES_CAL_DO_R {
        RES_CAL_DO_R::new((self.bits & 0x3f) as u8)
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
#[doc = "`reset()` method sets RESCAL_STATUS to value 0"]
impl crate::Resettable for RESCAL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
