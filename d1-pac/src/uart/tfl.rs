#[doc = "Register `tfl` reader"]
pub struct R(crate::R<TFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tfl` reader - TX FIFO Level"]
pub type TFL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - TX FIFO Level"]
    #[inline(always)]
    pub fn tfl(&self) -> TFL_R {
        TFL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "UART Transmit FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfl](index.html) module"]
pub struct TFL_SPEC;
impl crate::RegisterSpec for TFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfl::R](R) reader structure"]
impl crate::Readable for TFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tfl to value 0"]
impl crate::Resettable for TFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
