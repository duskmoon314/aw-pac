#[doc = "Register `RETITE_PC1_REG` reader"]
pub struct R(crate::R<RETITE_PC1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETITE_PC1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETITE_PC1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETITE_PC1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Retire Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RT_SIG` reader - Retire Signal"]
pub struct RT_SIG_R(crate::FieldReader<bool>);
impl RT_SIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RT_SIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RT_SIG_A::NOT_HAVE
    }
    #[doc = "Checks if the value of the field is `HAVE`"]
    #[inline(always)]
    pub fn is_have(&self) -> bool {
        **self == RT_SIG_A::HAVE
    }
}
impl core::ops::Deref for RT_SIG_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT_PC_H` reader - Retire PC\\[39:32\\]"]
pub struct RT_PC_H_R(crate::FieldReader<u8>);
impl RT_PC_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RT_PC_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RT_PC_H_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Retire Signal"]
    #[inline(always)]
    pub fn rt_sig(&self) -> RT_SIG_R {
        RT_SIG_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Retire PC\\[39:32\\]"]
    #[inline(always)]
    pub fn rt_pc_h(&self) -> RT_PC_H_R {
        RT_PC_H_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Retire PC1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retite_pc1_reg](index.html) module"]
pub struct RETITE_PC1_REG_SPEC;
impl crate::RegisterSpec for RETITE_PC1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retite_pc1_reg::R](R) reader structure"]
impl crate::Readable for RETITE_PC1_REG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RETITE_PC1_REG to value 0"]
impl crate::Resettable for RETITE_PC1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
