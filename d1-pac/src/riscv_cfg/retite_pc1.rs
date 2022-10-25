#[doc = "Register `retite_pc1` reader"]
pub struct R(crate::R<RETITE_PC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETITE_PC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETITE_PC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETITE_PC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `rt_pc_h` reader - Retire PC\\[39:32\\]"]
pub type RT_PC_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rt_sig` reader - Retire Signal"]
pub type RT_SIG_R = crate::BitReader<RT_SIG_A>;
#[doc = "Retire Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT_SIG_A {
    #[doc = "0: `0`"]
    NOT_HAVE = 0,
    #[doc = "1: `1`"]
    HAVE = 1,
}
impl From<RT_SIG_A> for bool {
    #[inline(always)]
    fn from(variant: RT_SIG_A) -> Self {
        variant as u8 != 0
    }
}
impl RT_SIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT_SIG_A {
        match self.bits {
            false => RT_SIG_A::NOT_HAVE,
            true => RT_SIG_A::HAVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_HAVE`"]
    #[inline(always)]
    pub fn is_not_have(&self) -> bool {
        *self == RT_SIG_A::NOT_HAVE
    }
    #[doc = "Checks if the value of the field is `HAVE`"]
    #[inline(always)]
    pub fn is_have(&self) -> bool {
        *self == RT_SIG_A::HAVE
    }
}
impl R {
    #[doc = "Bits 0:7 - Retire PC\\[39:32\\]"]
    #[inline(always)]
    pub fn rt_pc_h(&self) -> RT_PC_H_R {
        RT_PC_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Retire Signal"]
    #[inline(always)]
    pub fn rt_sig(&self) -> RT_SIG_R {
        RT_SIG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Retire PC1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retite_pc1](index.html) module"]
pub struct RETITE_PC1_SPEC;
impl crate::RegisterSpec for RETITE_PC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retite_pc1::R](R) reader structure"]
impl crate::Readable for RETITE_PC1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets retite_pc1 to value 0"]
impl crate::Resettable for RETITE_PC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
