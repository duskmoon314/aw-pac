#[doc = "Register `rescal_status` reader"]
pub type R = crate::R<RESCAL_STATUS_SPEC>;
#[doc = "Field `res_cal_do` reader - RESCAL Calibration Results Output"]
pub type RES_CAL_DO_R = crate::FieldReader;
#[doc = "Field `cout` reader - Calibration Circuits Analog COmpare Output"]
pub type COUT_R = crate::BitReader;
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
#[doc = "Resistor Calibration Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rescal_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESCAL_STATUS_SPEC;
impl crate::RegisterSpec for RESCAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rescal_status::R`](R) reader structure"]
impl crate::Readable for RESCAL_STATUS_SPEC {}
#[doc = "`reset()` method sets rescal_status to value 0"]
impl crate::Resettable for RESCAL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
