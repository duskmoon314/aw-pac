#[doc = "Register `ledc_fifo_data%s` reader"]
pub type R = crate::R<LEDC_FIFO_DATA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<LEDC_FIFO_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LEDC FIFO Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ledc_fifo_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_FIFO_DATA_SPEC;
impl crate::RegisterSpec for LEDC_FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_fifo_data::R`](R) reader structure"]
impl crate::Readable for LEDC_FIFO_DATA_SPEC {}
#[doc = "`reset()` method sets ledc_fifo_data%s to value 0"]
impl crate::Resettable for LEDC_FIFO_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
