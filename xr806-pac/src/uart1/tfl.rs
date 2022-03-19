#[doc = "Register `TFL` reader"]
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
pub struct TFL_R(crate::FieldReader<u8, u8>);
impl TFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - TX FIFO Level"]
    #[inline(always)]
    pub fn tfl(&self) -> TFL_R {
        TFL_R::new((self.bits & 0x7f) as u8)
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
#[doc = "`reset()` method sets TFL to value 0"]
impl crate::Resettable for TFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
