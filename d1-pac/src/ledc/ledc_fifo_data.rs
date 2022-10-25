#[doc = "Register `ledc_fifo_data%s` reader"]
pub struct R(crate::R<LEDC_FIFO_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDC_FIFO_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDC_FIFO_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDC_FIFO_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LEDC FIFO Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledc_fifo_data](index.html) module"]
pub struct LEDC_FIFO_DATA_SPEC;
impl crate::RegisterSpec for LEDC_FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledc_fifo_data::R](R) reader structure"]
impl crate::Readable for LEDC_FIFO_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ledc_fifo_data%s to value 0"]
impl crate::Resettable for LEDC_FIFO_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
