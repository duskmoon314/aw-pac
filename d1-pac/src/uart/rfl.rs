#[doc = "Register `RFL` reader"]
pub struct R(crate::R<RFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rfl` reader - RX FIFO Level"]
pub struct RFL_R(crate::FieldReader<u16>);
impl RFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFL_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - RX FIFO Level"]
    #[inline(always)]
    pub fn rfl(&self) -> RFL_R {
        RFL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "UART Receive FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfl](index.html) module"]
pub struct RFL_SPEC;
impl crate::RegisterSpec for RFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfl::R](R) reader structure"]
impl crate::Readable for RFL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFL to value 0"]
impl crate::Resettable for RFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
