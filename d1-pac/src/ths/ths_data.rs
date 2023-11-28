#[doc = "Register `ths_data` reader"]
pub type R = crate::R<THS_DATA_SPEC>;
#[doc = "Field `ths_data` reader - Temperature measurement data of sensor"]
pub type THS_DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Temperature measurement data of sensor"]
    #[inline(always)]
    pub fn ths_data(&self) -> THS_DATA_R {
        THS_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "THS Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_DATA_SPEC;
impl crate::RegisterSpec for THS_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_data::R`](R) reader structure"]
impl crate::Readable for THS_DATA_SPEC {}
#[doc = "`reset()` method sets ths_data to value 0"]
impl crate::Resettable for THS_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
