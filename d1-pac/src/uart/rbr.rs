#[doc = "Register `rbr` reader"]
pub struct R(crate::R<RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rbr` reader - "]
pub type RBR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr](index.html) module"]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbr::R](R) reader structure"]
impl crate::Readable for RBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rbr to value 0"]
impl crate::Resettable for RBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
