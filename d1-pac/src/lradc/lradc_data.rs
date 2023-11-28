#[doc = "Register `lradc_data` reader"]
pub type R = crate::R<LRADC_DATA_SPEC>;
#[doc = "Field `lradc_data` reader - LRADC Data"]
pub type LRADC_DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - LRADC Data"]
    #[inline(always)]
    pub fn lradc_data(&self) -> LRADC_DATA_R {
        LRADC_DATA_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LRADC Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LRADC_DATA_SPEC;
impl crate::RegisterSpec for LRADC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lradc_data::R`](R) reader structure"]
impl crate::Readable for LRADC_DATA_SPEC {}
#[doc = "`reset()` method sets lradc_data to value 0"]
impl crate::Resettable for LRADC_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
