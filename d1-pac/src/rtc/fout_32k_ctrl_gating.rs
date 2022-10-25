#[doc = "Register `fout_32k_ctrl_gating` reader"]
pub struct R(crate::R<FOUT_32K_CTRL_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOUT_32K_CTRL_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOUT_32K_CTRL_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOUT_32K_CTRL_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fout_32k_ctrl_gating` writer"]
pub struct W(crate::W<FOUT_32K_CTRL_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FOUT_32K_CTRL_GATING_SPEC>;
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
impl From<crate::W<FOUT_32K_CTRL_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FOUT_32K_CTRL_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fanout_32k_gating` reader - LOSC out gating enable\n\nConfiguration of LOSC output, and there is no LOSC output by default."]
pub type FANOUT_32K_GATING_R = crate::BitReader<FANOUT_32K_GATING_A>;
#[doc = "LOSC out gating enable\n\nConfiguration of LOSC output, and there is no LOSC output by default.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FANOUT_32K_GATING_A {
    #[doc = "0: Mask LOSC output gating"]
    MASK = 0,
    #[doc = "1: Enable LOSC output gating"]
    ENABLE = 1,
}
impl From<FANOUT_32K_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: FANOUT_32K_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl FANOUT_32K_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FANOUT_32K_GATING_A {
        match self.bits {
            false => FANOUT_32K_GATING_A::MASK,
            true => FANOUT_32K_GATING_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == FANOUT_32K_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FANOUT_32K_GATING_A::ENABLE
    }
}
#[doc = "Field `fanout_32k_gating` writer - LOSC out gating enable\n\nConfiguration of LOSC output, and there is no LOSC output by default."]
pub type FANOUT_32K_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FOUT_32K_CTRL_GATING_SPEC, FANOUT_32K_GATING_A, O>;
impl<'a, const O: u8> FANOUT_32K_GATING_W<'a, O> {
    #[doc = "Mask LOSC output gating"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(FANOUT_32K_GATING_A::MASK)
    }
    #[doc = "Enable LOSC output gating"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FANOUT_32K_GATING_A::ENABLE)
    }
}
#[doc = "Field `losc_out_src_sel` reader - LOSC output source select"]
pub type LOSC_OUT_SRC_SEL_R = crate::FieldReader<u8, LOSC_OUT_SRC_SEL_A>;
#[doc = "LOSC output source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOSC_OUT_SRC_SEL_A {
    #[doc = "0: RTC_32K (select by RC_CLK_SRC_SEL LOSC_SRC_SEL)"]
    RTC_32K = 0,
    #[doc = "1: LOSC"]
    LOSC = 1,
    #[doc = "2: HOSC divided 32K"]
    HOSC = 2,
}
impl From<LOSC_OUT_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LOSC_OUT_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl LOSC_OUT_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOSC_OUT_SRC_SEL_A> {
        match self.bits {
            0 => Some(LOSC_OUT_SRC_SEL_A::RTC_32K),
            1 => Some(LOSC_OUT_SRC_SEL_A::LOSC),
            2 => Some(LOSC_OUT_SRC_SEL_A::HOSC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_32K`"]
    #[inline(always)]
    pub fn is_rtc_32k(&self) -> bool {
        *self == LOSC_OUT_SRC_SEL_A::RTC_32K
    }
    #[doc = "Checks if the value of the field is `LOSC`"]
    #[inline(always)]
    pub fn is_losc(&self) -> bool {
        *self == LOSC_OUT_SRC_SEL_A::LOSC
    }
    #[doc = "Checks if the value of the field is `HOSC`"]
    #[inline(always)]
    pub fn is_hosc(&self) -> bool {
        *self == LOSC_OUT_SRC_SEL_A::HOSC
    }
}
#[doc = "Field `losc_out_src_sel` writer - LOSC output source select"]
pub type LOSC_OUT_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FOUT_32K_CTRL_GATING_SPEC, u8, LOSC_OUT_SRC_SEL_A, 2, O>;
impl<'a, const O: u8> LOSC_OUT_SRC_SEL_W<'a, O> {
    #[doc = "RTC_32K (select by RC_CLK_SRC_SEL LOSC_SRC_SEL)"]
    #[inline(always)]
    pub fn rtc_32k(self) -> &'a mut W {
        self.variant(LOSC_OUT_SRC_SEL_A::RTC_32K)
    }
    #[doc = "LOSC"]
    #[inline(always)]
    pub fn losc(self) -> &'a mut W {
        self.variant(LOSC_OUT_SRC_SEL_A::LOSC)
    }
    #[doc = "HOSC divided 32K"]
    #[inline(always)]
    pub fn hosc(self) -> &'a mut W {
        self.variant(LOSC_OUT_SRC_SEL_A::HOSC)
    }
}
#[doc = "Field `hosc_to_32k_divider_enable` reader - HOSC to 32k divider enable"]
pub type HOSC_TO_32K_DIVIDER_ENABLE_R = crate::BitReader<HOSC_TO_32K_DIVIDER_ENABLE_A>;
#[doc = "HOSC to 32k divider enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOSC_TO_32K_DIVIDER_ENABLE_A {
    #[doc = "0: Disable the hosc 24M to 32K divider circuit"]
    DISABLE = 0,
    #[doc = "1: Enable the hosc 24M to 32K divider circuit"]
    ENABLE = 1,
}
impl From<HOSC_TO_32K_DIVIDER_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: HOSC_TO_32K_DIVIDER_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl HOSC_TO_32K_DIVIDER_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOSC_TO_32K_DIVIDER_ENABLE_A {
        match self.bits {
            false => HOSC_TO_32K_DIVIDER_ENABLE_A::DISABLE,
            true => HOSC_TO_32K_DIVIDER_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HOSC_TO_32K_DIVIDER_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HOSC_TO_32K_DIVIDER_ENABLE_A::ENABLE
    }
}
#[doc = "Field `hosc_to_32k_divider_enable` writer - HOSC to 32k divider enable"]
pub type HOSC_TO_32K_DIVIDER_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FOUT_32K_CTRL_GATING_SPEC, HOSC_TO_32K_DIVIDER_ENABLE_A, O>;
impl<'a, const O: u8> HOSC_TO_32K_DIVIDER_ENABLE_W<'a, O> {
    #[doc = "Disable the hosc 24M to 32K divider circuit"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HOSC_TO_32K_DIVIDER_ENABLE_A::DISABLE)
    }
    #[doc = "Enable the hosc 24M to 32K divider circuit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HOSC_TO_32K_DIVIDER_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - LOSC out gating enable\n\nConfiguration of LOSC output, and there is no LOSC output by default."]
    #[inline(always)]
    pub fn fanout_32k_gating(&self) -> FANOUT_32K_GATING_R {
        FANOUT_32K_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - LOSC output source select"]
    #[inline(always)]
    pub fn losc_out_src_sel(&self) -> LOSC_OUT_SRC_SEL_R {
        LOSC_OUT_SRC_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - HOSC to 32k divider enable"]
    #[inline(always)]
    pub fn hosc_to_32k_divider_enable(&self) -> HOSC_TO_32K_DIVIDER_ENABLE_R {
        HOSC_TO_32K_DIVIDER_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOSC out gating enable\n\nConfiguration of LOSC output, and there is no LOSC output by default."]
    #[inline(always)]
    #[must_use]
    pub fn fanout_32k_gating(&mut self) -> FANOUT_32K_GATING_W<0> {
        FANOUT_32K_GATING_W::new(self)
    }
    #[doc = "Bits 1:2 - LOSC output source select"]
    #[inline(always)]
    #[must_use]
    pub fn losc_out_src_sel(&mut self) -> LOSC_OUT_SRC_SEL_W<1> {
        LOSC_OUT_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 16 - HOSC to 32k divider enable"]
    #[inline(always)]
    #[must_use]
    pub fn hosc_to_32k_divider_enable(&mut self) -> HOSC_TO_32K_DIVIDER_ENABLE_W<16> {
        HOSC_TO_32K_DIVIDER_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32K Fanout Control Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fout_32k_ctrl_gating](index.html) module"]
pub struct FOUT_32K_CTRL_GATING_SPEC;
impl crate::RegisterSpec for FOUT_32K_CTRL_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fout_32k_ctrl_gating::R](R) reader structure"]
impl crate::Readable for FOUT_32K_CTRL_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fout_32k_ctrl_gating::W](W) writer structure"]
impl crate::Writable for FOUT_32K_CTRL_GATING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fout_32k_ctrl_gating to value 0"]
impl crate::Resettable for FOUT_32K_CTRL_GATING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
