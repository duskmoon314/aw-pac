#[doc = "Register `EMMC_DDR_SBIT_DET` reader"]
pub struct R(crate::R<EMMC_DDR_SBIT_DET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_DDR_SBIT_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_DDR_SBIT_DET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_DDR_SBIT_DET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_DDR_SBIT_DET` writer"]
pub struct W(crate::W<EMMC_DDR_SBIT_DET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_DDR_SBIT_DET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EMMC_DDR_SBIT_DET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_DDR_SBIT_DET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HS400 Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS400_MD_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HS400_MD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS400_MD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS400_MD_EN` reader - HS400 Mode Enable"]
pub type HS400_MD_EN_R = crate::BitReader<HS400_MD_EN_A>;
impl HS400_MD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS400_MD_EN_A {
        match self.bits {
            false => HS400_MD_EN_A::DISABLED,
            true => HS400_MD_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HS400_MD_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HS400_MD_EN_A::ENABLED
    }
}
#[doc = "Field `HS400_MD_EN` writer - HS400 Mode Enable"]
pub type HS400_MD_EN_W<'a> = crate::BitWriter<'a, u32, EMMC_DDR_SBIT_DET_SPEC, HS400_MD_EN_A, 31>;
impl<'a> HS400_MD_EN_W<'a> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HS400_MD_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HS400_MD_EN_A::ENABLED)
    }
}
#[doc = "Control for start bit detection mechanism inside mstorage based on duration of start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALF_START_BIT_A {
    #[doc = "0: Full cycle"]
    FULL = 0,
    #[doc = "1: Less than one full cycle"]
    LESS = 1,
}
impl From<HALF_START_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_START_BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALF_START_BIT` reader - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
pub type HALF_START_BIT_R = crate::BitReader<HALF_START_BIT_A>;
impl HALF_START_BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALF_START_BIT_A {
        match self.bits {
            false => HALF_START_BIT_A::FULL,
            true => HALF_START_BIT_A::LESS,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == HALF_START_BIT_A::FULL
    }
    #[doc = "Checks if the value of the field is `LESS`"]
    #[inline(always)]
    pub fn is_less(&self) -> bool {
        *self == HALF_START_BIT_A::LESS
    }
}
#[doc = "Field `HALF_START_BIT` writer - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
pub type HALF_START_BIT_W<'a> =
    crate::BitWriter<'a, u32, EMMC_DDR_SBIT_DET_SPEC, HALF_START_BIT_A, 0>;
impl<'a> HALF_START_BIT_W<'a> {
    #[doc = "Full cycle"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(HALF_START_BIT_A::FULL)
    }
    #[doc = "Less than one full cycle"]
    #[inline(always)]
    pub fn less(self) -> &'a mut W {
        self.variant(HALF_START_BIT_A::LESS)
    }
}
impl R {
    #[doc = "Bit 31 - HS400 Mode Enable"]
    #[inline(always)]
    pub fn hs400_md_en(&self) -> HS400_MD_EN_R {
        HS400_MD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 0 - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
    #[inline(always)]
    pub fn half_start_bit(&self) -> HALF_START_BIT_R {
        HALF_START_BIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - HS400 Mode Enable"]
    #[inline(always)]
    pub fn hs400_md_en(&mut self) -> HS400_MD_EN_W {
        HS400_MD_EN_W::new(self)
    }
    #[doc = "Bit 0 - Control for start bit detection mechanism inside mstorage based on duration of start bit"]
    #[inline(always)]
    pub fn half_start_bit(&mut self) -> HALF_START_BIT_W {
        HALF_START_BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eMMC4.5 DDR Start Bit Detection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_ddr_sbit_det](index.html) module"]
pub struct EMMC_DDR_SBIT_DET_SPEC;
impl crate::RegisterSpec for EMMC_DDR_SBIT_DET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_ddr_sbit_det::R](R) reader structure"]
impl crate::Readable for EMMC_DDR_SBIT_DET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_ddr_sbit_det::W](W) writer structure"]
impl crate::Writable for EMMC_DDR_SBIT_DET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_DDR_SBIT_DET to value 0"]
impl crate::Resettable for EMMC_DDR_SBIT_DET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
