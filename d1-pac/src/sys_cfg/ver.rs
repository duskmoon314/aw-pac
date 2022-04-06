#[doc = "Register `VER` reader"]
pub struct R(crate::R<VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOOT_SEL_PAD_STA` reader - "]
pub struct BOOT_SEL_PAD_STA_R(crate::FieldReader<u8, u8>);
impl BOOT_SEL_PAD_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BOOT_SEL_PAD_STA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_SEL_PAD_STA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fel Select Pin Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEL_SEL_PAD_STA_A {
    #[doc = "0: `0`"]
    RUN_FEL = 0,
    #[doc = "1: `1`"]
    TRY_MEDIA_BOOT = 1,
}
impl From<FEL_SEL_PAD_STA_A> for bool {
    #[inline(always)]
    fn from(variant: FEL_SEL_PAD_STA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEL_SEL_PAD_STA` reader - Fel Select Pin Status"]
pub struct FEL_SEL_PAD_STA_R(crate::FieldReader<bool, FEL_SEL_PAD_STA_A>);
impl FEL_SEL_PAD_STA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEL_SEL_PAD_STA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEL_SEL_PAD_STA_A {
        match self.bits {
            false => FEL_SEL_PAD_STA_A::RUN_FEL,
            true => FEL_SEL_PAD_STA_A::TRY_MEDIA_BOOT,
        }
    }
    #[doc = "Checks if the value of the field is `RUN_FEL`"]
    #[inline(always)]
    pub fn is_run_fel(&self) -> bool {
        **self == FEL_SEL_PAD_STA_A::RUN_FEL
    }
    #[doc = "Checks if the value of the field is `TRY_MEDIA_BOOT`"]
    #[inline(always)]
    pub fn is_try_media_boot(&self) -> bool {
        **self == FEL_SEL_PAD_STA_A::TRY_MEDIA_BOOT
    }
}
impl core::ops::Deref for FEL_SEL_PAD_STA_R {
    type Target = crate::FieldReader<bool, FEL_SEL_PAD_STA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn boot_sel_pad_sta(&self) -> BOOT_SEL_PAD_STA_R {
        BOOT_SEL_PAD_STA_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 8 - Fel Select Pin Status"]
    #[inline(always)]
    pub fn fel_sel_pad_sta(&self) -> FEL_SEL_PAD_STA_R {
        FEL_SEL_PAD_STA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ver](index.html) module"]
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ver::R](R) reader structure"]
impl crate::Readable for VER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VER to value 0"]
impl crate::Resettable for VER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
