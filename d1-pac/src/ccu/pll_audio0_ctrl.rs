#[doc = "Register `pll_audio0_ctrl` reader"]
pub struct R(crate::R<PLL_AUDIO0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_AUDIO0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_AUDIO0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_AUDIO0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pll_audio0_ctrl` writer"]
pub struct W(crate::W<PLL_AUDIO0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_AUDIO0_CTRL_SPEC>;
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
impl From<crate::W<PLL_AUDIO0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_AUDIO0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pll_output_div2` reader - PLL Output Div M0"]
pub type PLL_OUTPUT_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `pll_output_div2` writer - PLL Output Div M0"]
pub type PLL_OUTPUT_DIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, bool, O>;
#[doc = "Field `pll_input_div2` reader - PLL Input Div M1"]
pub type PLL_INPUT_DIV2_R = crate::BitReader<bool>;
#[doc = "Field `pll_input_div2` writer - PLL Input Div M1"]
pub type PLL_INPUT_DIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, bool, O>;
#[doc = "Field `pll_lock_mdsel` reader - PLL Lock Level"]
pub type PLL_LOCK_MDSEL_R = crate::BitReader<PLL_LOCK_MDSEL_A>;
#[doc = "PLL Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_LOCK_MDSEL_A {
    #[doc = "0: `0`"]
    CC_24_26 = 0,
    #[doc = "1: `1`"]
    CC_23_27 = 1,
}
impl From<PLL_LOCK_MDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_MDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_LOCK_MDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_MDSEL_A {
        match self.bits {
            false => PLL_LOCK_MDSEL_A::CC_24_26,
            true => PLL_LOCK_MDSEL_A::CC_23_27,
        }
    }
    #[doc = "Checks if the value of the field is `CC_24_26`"]
    #[inline(always)]
    pub fn is_cc_24_26(&self) -> bool {
        *self == PLL_LOCK_MDSEL_A::CC_24_26
    }
    #[doc = "Checks if the value of the field is `CC_23_27`"]
    #[inline(always)]
    pub fn is_cc_23_27(&self) -> bool {
        *self == PLL_LOCK_MDSEL_A::CC_23_27
    }
}
#[doc = "Field `pll_lock_mdsel` writer - PLL Lock Level"]
pub type PLL_LOCK_MDSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, PLL_LOCK_MDSEL_A, O>;
impl<'a, const O: u8> PLL_LOCK_MDSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cc_24_26(self) -> &'a mut W {
        self.variant(PLL_LOCK_MDSEL_A::CC_24_26)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cc_23_27(self) -> &'a mut W {
        self.variant(PLL_LOCK_MDSEL_A::CC_23_27)
    }
}
#[doc = "Field `pll_unlock_mdsel` reader - PLL Unlock Level"]
pub type PLL_UNLOCK_MDSEL_R = crate::FieldReader<u8, PLL_UNLOCK_MDSEL_A>;
#[doc = "PLL Unlock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL_UNLOCK_MDSEL_A {
    #[doc = "0: `0`"]
    CC_21_29 = 0,
    #[doc = "1: `1`"]
    CC_22_28 = 1,
    #[doc = "2: `10`"]
    CC_20_30 = 2,
}
impl From<PLL_UNLOCK_MDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_UNLOCK_MDSEL_A) -> Self {
        variant as _
    }
}
impl PLL_UNLOCK_MDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLL_UNLOCK_MDSEL_A> {
        match self.bits {
            0 => Some(PLL_UNLOCK_MDSEL_A::CC_21_29),
            1 => Some(PLL_UNLOCK_MDSEL_A::CC_22_28),
            2 => Some(PLL_UNLOCK_MDSEL_A::CC_20_30),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CC_21_29`"]
    #[inline(always)]
    pub fn is_cc_21_29(&self) -> bool {
        *self == PLL_UNLOCK_MDSEL_A::CC_21_29
    }
    #[doc = "Checks if the value of the field is `CC_22_28`"]
    #[inline(always)]
    pub fn is_cc_22_28(&self) -> bool {
        *self == PLL_UNLOCK_MDSEL_A::CC_22_28
    }
    #[doc = "Checks if the value of the field is `CC_20_30`"]
    #[inline(always)]
    pub fn is_cc_20_30(&self) -> bool {
        *self == PLL_UNLOCK_MDSEL_A::CC_20_30
    }
}
#[doc = "Field `pll_unlock_mdsel` writer - PLL Unlock Level"]
pub type PLL_UNLOCK_MDSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, u8, PLL_UNLOCK_MDSEL_A, 2, O>;
impl<'a, const O: u8> PLL_UNLOCK_MDSEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn cc_21_29(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_21_29)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cc_22_28(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_22_28)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cc_20_30(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_20_30)
    }
}
#[doc = "Field `pll_n` reader - PLL N"]
pub type PLL_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_n` writer - PLL N"]
pub type PLL_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `pll_p` reader - PLL Post-div P"]
pub type PLL_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pll_p` writer - PLL Post-div P"]
pub type PLL_P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `pll_sdm_en` reader - PLL SDM Enable"]
pub type PLL_SDM_EN_R = crate::BitReader<PLL_SDM_EN_A>;
#[doc = "PLL SDM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_SDM_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_SDM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_SDM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_SDM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_SDM_EN_A {
        match self.bits {
            false => PLL_SDM_EN_A::DISABLE,
            true => PLL_SDM_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_SDM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_SDM_EN_A::ENABLE
    }
}
#[doc = "Field `pll_sdm_en` writer - PLL SDM Enable"]
pub type PLL_SDM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, PLL_SDM_EN_A, O>;
impl<'a, const O: u8> PLL_SDM_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_SDM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_SDM_EN_A::ENABLE)
    }
}
#[doc = "Field `pll_output_gate` reader - PLL Output Gating Enable"]
pub type PLL_OUTPUT_GATE_R = crate::BitReader<PLL_OUTPUT_GATE_A>;
#[doc = "PLL Output Gating Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_OUTPUT_GATE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_OUTPUT_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_OUTPUT_GATE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_OUTPUT_GATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_OUTPUT_GATE_A {
        match self.bits {
            false => PLL_OUTPUT_GATE_A::DISABLE,
            true => PLL_OUTPUT_GATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_OUTPUT_GATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_OUTPUT_GATE_A::ENABLE
    }
}
#[doc = "Field `pll_output_gate` writer - PLL Output Gating Enable"]
pub type PLL_OUTPUT_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, PLL_OUTPUT_GATE_A, O>;
impl<'a, const O: u8> PLL_OUTPUT_GATE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_OUTPUT_GATE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_OUTPUT_GATE_A::ENABLE)
    }
}
#[doc = "Field `lock` reader - PLL Lock Status"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "PLL Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `lock_enable` reader - Lock Enable"]
pub type LOCK_ENABLE_R = crate::BitReader<LOCK_ENABLE_A>;
#[doc = "Lock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LOCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_ENABLE_A {
        match self.bits {
            false => LOCK_ENABLE_A::DISABLE,
            true => LOCK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOCK_ENABLE_A::ENABLE
    }
}
#[doc = "Field `lock_enable` writer - Lock Enable"]
pub type LOCK_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, LOCK_ENABLE_A, O>;
impl<'a, const O: u8> LOCK_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `pll_ldo_en` reader - LDO Enable"]
pub type PLL_LDO_EN_R = crate::BitReader<PLL_LDO_EN_A>;
#[doc = "LDO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_LDO_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_LDO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LDO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_LDO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LDO_EN_A {
        match self.bits {
            false => PLL_LDO_EN_A::DISABLE,
            true => PLL_LDO_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_LDO_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_LDO_EN_A::ENABLE
    }
}
#[doc = "Field `pll_ldo_en` writer - LDO Enable"]
pub type PLL_LDO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, PLL_LDO_EN_A, O>;
impl<'a, const O: u8> PLL_LDO_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_LDO_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_LDO_EN_A::ENABLE)
    }
}
#[doc = "Field `pll_en` reader - PLL Enable"]
pub type PLL_EN_R = crate::BitReader<PLL_EN_A>;
#[doc = "PLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<PLL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_EN_A {
        match self.bits {
            false => PLL_EN_A::DISABLE,
            true => PLL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLL_EN_A::ENABLE
    }
}
#[doc = "Field `pll_en` writer - PLL Enable"]
pub type PLL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_AUDIO0_CTRL_SPEC, PLL_EN_A, O>;
impl<'a, const O: u8> PLL_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLL_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - PLL Output Div M0"]
    #[inline(always)]
    pub fn pll_output_div2(&self) -> PLL_OUTPUT_DIV2_R {
        PLL_OUTPUT_DIV2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Input Div M1"]
    #[inline(always)]
    pub fn pll_input_div2(&self) -> PLL_INPUT_DIV2_R {
        PLL_INPUT_DIV2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Lock Level"]
    #[inline(always)]
    pub fn pll_lock_mdsel(&self) -> PLL_LOCK_MDSEL_R {
        PLL_LOCK_MDSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PLL Unlock Level"]
    #[inline(always)]
    pub fn pll_unlock_mdsel(&self) -> PLL_UNLOCK_MDSEL_R {
        PLL_UNLOCK_MDSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - PLL N"]
    #[inline(always)]
    pub fn pll_n(&self) -> PLL_N_R {
        PLL_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - PLL Post-div P"]
    #[inline(always)]
    pub fn pll_p(&self) -> PLL_P_R {
        PLL_P_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - PLL SDM Enable"]
    #[inline(always)]
    pub fn pll_sdm_en(&self) -> PLL_SDM_EN_R {
        PLL_SDM_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    pub fn pll_output_gate(&self) -> PLL_OUTPUT_GATE_R {
        PLL_OUTPUT_GATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock Enable"]
    #[inline(always)]
    pub fn lock_enable(&self) -> LOCK_ENABLE_R {
        LOCK_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LDO Enable"]
    #[inline(always)]
    pub fn pll_ldo_en(&self) -> PLL_LDO_EN_R {
        PLL_LDO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&self) -> PLL_EN_R {
        PLL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Output Div M0"]
    #[inline(always)]
    #[must_use]
    pub fn pll_output_div2(&mut self) -> PLL_OUTPUT_DIV2_W<0> {
        PLL_OUTPUT_DIV2_W::new(self)
    }
    #[doc = "Bit 1 - PLL Input Div M1"]
    #[inline(always)]
    #[must_use]
    pub fn pll_input_div2(&mut self) -> PLL_INPUT_DIV2_W<1> {
        PLL_INPUT_DIV2_W::new(self)
    }
    #[doc = "Bit 5 - PLL Lock Level"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lock_mdsel(&mut self) -> PLL_LOCK_MDSEL_W<5> {
        PLL_LOCK_MDSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - PLL Unlock Level"]
    #[inline(always)]
    #[must_use]
    pub fn pll_unlock_mdsel(&mut self) -> PLL_UNLOCK_MDSEL_W<6> {
        PLL_UNLOCK_MDSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - PLL N"]
    #[inline(always)]
    #[must_use]
    pub fn pll_n(&mut self) -> PLL_N_W<8> {
        PLL_N_W::new(self)
    }
    #[doc = "Bits 16:21 - PLL Post-div P"]
    #[inline(always)]
    #[must_use]
    pub fn pll_p(&mut self) -> PLL_P_W<16> {
        PLL_P_W::new(self)
    }
    #[doc = "Bit 24 - PLL SDM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_sdm_en(&mut self) -> PLL_SDM_EN_W<24> {
        PLL_SDM_EN_W::new(self)
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_output_gate(&mut self) -> PLL_OUTPUT_GATE_W<27> {
        PLL_OUTPUT_GATE_W::new(self)
    }
    #[doc = "Bit 29 - Lock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_enable(&mut self) -> LOCK_ENABLE_W<29> {
        LOCK_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - LDO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_ldo_en(&mut self) -> PLL_LDO_EN_W<30> {
        PLL_LDO_EN_W::new(self)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pll_en(&mut self) -> PLL_EN_W<31> {
        PLL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_AUDIO0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_audio0_ctrl](index.html) module"]
pub struct PLL_AUDIO0_CTRL_SPEC;
impl crate::RegisterSpec for PLL_AUDIO0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_audio0_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_AUDIO0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_audio0_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_AUDIO0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_audio0_ctrl to value 0"]
impl crate::Resettable for PLL_AUDIO0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
