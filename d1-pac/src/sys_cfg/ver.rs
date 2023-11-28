#[doc = "Register `ver` reader"]
pub type R = crate::R<VER_SPEC>;
#[doc = "Field `fel_sel_pad_sta` reader - Fel Select Pin Status"]
pub type FEL_SEL_PAD_STA_R = crate::BitReader<FEL_SEL_PAD_STA_A>;
#[doc = "Fel Select Pin Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl FEL_SEL_PAD_STA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FEL_SEL_PAD_STA_A {
        match self.bits {
            false => FEL_SEL_PAD_STA_A::RUN_FEL,
            true => FEL_SEL_PAD_STA_A::TRY_MEDIA_BOOT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_run_fel(&self) -> bool {
        *self == FEL_SEL_PAD_STA_A::RUN_FEL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_try_media_boot(&self) -> bool {
        *self == FEL_SEL_PAD_STA_A::TRY_MEDIA_BOOT
    }
}
#[doc = "Field `boot_sel_pad_sta` reader - "]
pub type BOOT_SEL_PAD_STA_R = crate::FieldReader;
impl R {
    #[doc = "Bit 8 - Fel Select Pin Status"]
    #[inline(always)]
    pub fn fel_sel_pad_sta(&self) -> FEL_SEL_PAD_STA_R {
        FEL_SEL_PAD_STA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn boot_sel_pad_sta(&self) -> BOOT_SEL_PAD_STA_R {
        BOOT_SEL_PAD_STA_R::new(((self.bits >> 11) & 3) as u8)
    }
}
#[doc = "Version Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VER_SPEC {}
#[doc = "`reset()` method sets ver to value 0"]
impl crate::Resettable for VER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
