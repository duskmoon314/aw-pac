#[doc = "Register `PLL_CPU_CTRL` reader"]
pub struct R(crate::R<PLL_CPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CPU_CTRL` writer"]
pub struct W(crate::W<PLL_CPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CPU_CTRL_SPEC>;
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
impl From<crate::W<PLL_CPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLL Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PLL_EN` reader - PLL Enable"]
pub struct PLL_EN_R(crate::FieldReader<bool, PLL_EN_A>);
impl PLL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLL_EN_A::ENABLE
    }
}
impl core::ops::Deref for PLL_EN_R {
    type Target = crate::FieldReader<bool, PLL_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_EN` writer - PLL Enable"]
pub struct PLL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "LDO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PLL_LDO_EN` reader - LDO Enable"]
pub struct PLL_LDO_EN_R(crate::FieldReader<bool, PLL_LDO_EN_A>);
impl PLL_LDO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LDO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_LDO_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLL_LDO_EN_A::ENABLE
    }
}
impl core::ops::Deref for PLL_LDO_EN_R {
    type Target = crate::FieldReader<bool, PLL_LDO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LDO_EN` writer - LDO Enable"]
pub struct PLL_LDO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LDO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_LDO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Lock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<LOCK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK_EN` reader - Lock Enable"]
pub struct LOCK_EN_R(crate::FieldReader<bool, LOCK_EN_A>);
impl LOCK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_EN_A {
        match self.bits {
            false => LOCK_EN_A::DISABLE,
            true => LOCK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == LOCK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LOCK_EN_A::ENABLE
    }
}
impl core::ops::Deref for LOCK_EN_R {
    type Target = crate::FieldReader<bool, LOCK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_EN` writer - Lock Enable"]
pub struct LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "PLL Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: It indicates that the PLL has been stable."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - PLL Lock Status"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LOCK_A::LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PLL Output Gating Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PLL_OUTPUT_GATE` reader - PLL Output Gating Enable"]
pub struct PLL_OUTPUT_GATE_R(crate::FieldReader<bool, PLL_OUTPUT_GATE_A>);
impl PLL_OUTPUT_GATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_OUTPUT_GATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_OUTPUT_GATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLL_OUTPUT_GATE_A::ENABLE
    }
}
impl core::ops::Deref for PLL_OUTPUT_GATE_R {
    type Target = crate::FieldReader<bool, PLL_OUTPUT_GATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_OUTPUT_GATE` writer - PLL Output Gating Enable"]
pub struct PLL_OUTPUT_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_OUTPUT_GATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_OUTPUT_GATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PLL_LOCK_TIME` reader - PLL Lock Time\n\nThe bit indicates the step amplitude from one frequency to another."]
pub struct PLL_LOCK_TIME_R(crate::FieldReader<u8, u8>);
impl PLL_LOCK_TIME_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_LOCK_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_LOCK_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LOCK_TIME` writer - PLL Lock Time\n\nThe bit indicates the step amplitude from one frequency to another."]
pub struct PLL_LOCK_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `PLL_N` reader - PLL N\n\nN = PLL_N + 1\n\nPLL_N is from 0 to 254"]
pub struct PLL_N_R(crate::FieldReader<u8, u8>);
impl PLL_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_N` writer - PLL N\n\nN = PLL_N + 1\n\nPLL_N is from 0 to 254"]
pub struct PLL_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "PLL Unlock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL_UNLOCK_MDSEL_A {
    #[doc = "0: 21 - 29 Clock Cycles"]
    CC_21_29 = 0,
    #[doc = "1: 22 - 28 Clock Cycles"]
    CC_22_28 = 1,
    #[doc = "2: 20 - 30 Clock Cycles"]
    CC_20_30 = 2,
}
impl From<PLL_UNLOCK_MDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_UNLOCK_MDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLL_UNLOCK_MDSEL` reader - PLL Unlock Level"]
pub struct PLL_UNLOCK_MDSEL_R(crate::FieldReader<u8, PLL_UNLOCK_MDSEL_A>);
impl PLL_UNLOCK_MDSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_UNLOCK_MDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_UNLOCK_MDSEL_A::CC_21_29
    }
    #[doc = "Checks if the value of the field is `CC_22_28`"]
    #[inline(always)]
    pub fn is_cc_22_28(&self) -> bool {
        **self == PLL_UNLOCK_MDSEL_A::CC_22_28
    }
    #[doc = "Checks if the value of the field is `CC_20_30`"]
    #[inline(always)]
    pub fn is_cc_20_30(&self) -> bool {
        **self == PLL_UNLOCK_MDSEL_A::CC_20_30
    }
}
impl core::ops::Deref for PLL_UNLOCK_MDSEL_R {
    type Target = crate::FieldReader<u8, PLL_UNLOCK_MDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_UNLOCK_MDSEL` writer - PLL Unlock Level"]
pub struct PLL_UNLOCK_MDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_UNLOCK_MDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_UNLOCK_MDSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "21 - 29 Clock Cycles"]
    #[inline(always)]
    pub fn cc_21_29(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_21_29)
    }
    #[doc = "22 - 28 Clock Cycles"]
    #[inline(always)]
    pub fn cc_22_28(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_22_28)
    }
    #[doc = "20 - 30 Clock Cycles"]
    #[inline(always)]
    pub fn cc_20_30(self) -> &'a mut W {
        self.variant(PLL_UNLOCK_MDSEL_A::CC_20_30)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "PLL Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_LOCK_MDSEL_A {
    #[doc = "0: 24 - 26 Clock Cycles"]
    CC_24_26 = 0,
    #[doc = "1: 23 - 27 Clock Cycles"]
    CC_23_27 = 1,
}
impl From<PLL_LOCK_MDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_MDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLL_LOCK_MDSEL` reader - PLL Lock Level"]
pub struct PLL_LOCK_MDSEL_R(crate::FieldReader<bool, PLL_LOCK_MDSEL_A>);
impl PLL_LOCK_MDSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LOCK_MDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_LOCK_MDSEL_A::CC_24_26
    }
    #[doc = "Checks if the value of the field is `CC_23_27`"]
    #[inline(always)]
    pub fn is_cc_23_27(&self) -> bool {
        **self == PLL_LOCK_MDSEL_A::CC_23_27
    }
}
impl core::ops::Deref for PLL_LOCK_MDSEL_R {
    type Target = crate::FieldReader<bool, PLL_LOCK_MDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LOCK_MDSEL` writer - PLL Lock Level"]
pub struct PLL_LOCK_MDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK_MDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_LOCK_MDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "24 - 26 Clock Cycles"]
    #[inline(always)]
    pub fn cc_24_26(self) -> &'a mut W {
        self.variant(PLL_LOCK_MDSEL_A::CC_24_26)
    }
    #[doc = "23 - 27 Clock Cycles"]
    #[inline(always)]
    pub fn cc_23_27(self) -> &'a mut W {
        self.variant(PLL_LOCK_MDSEL_A::CC_23_27)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PLL_M` reader - PLL M\n\nM = PLL_FACTOR_M + 1\n\nPLL_FACTOR_M is from 0 to 3"]
pub struct PLL_M_R(crate::FieldReader<u8, u8>);
impl PLL_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_M` writer - PLL M\n\nM = PLL_FACTOR_M + 1\n\nPLL_FACTOR_M is from 0 to 3"]
pub struct PLL_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&self) -> PLL_EN_R {
        PLL_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LDO Enable"]
    #[inline(always)]
    pub fn pll_ldo_en(&self) -> PLL_LDO_EN_R {
        PLL_LDO_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Enable"]
    #[inline(always)]
    pub fn lock_en(&self) -> LOCK_EN_R {
        LOCK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    pub fn pll_output_gate(&self) -> PLL_OUTPUT_GATE_R {
        PLL_OUTPUT_GATE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - PLL Lock Time\n\nThe bit indicates the step amplitude from one frequency to another."]
    #[inline(always)]
    pub fn pll_lock_time(&self) -> PLL_LOCK_TIME_R {
        PLL_LOCK_TIME_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - PLL N\n\nN = PLL_N + 1\n\nPLL_N is from 0 to 254"]
    #[inline(always)]
    pub fn pll_n(&self) -> PLL_N_R {
        PLL_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - PLL Unlock Level"]
    #[inline(always)]
    pub fn pll_unlock_mdsel(&self) -> PLL_UNLOCK_MDSEL_R {
        PLL_UNLOCK_MDSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - PLL Lock Level"]
    #[inline(always)]
    pub fn pll_lock_mdsel(&self) -> PLL_LOCK_MDSEL_R {
        PLL_LOCK_MDSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - PLL M\n\nM = PLL_FACTOR_M + 1\n\nPLL_FACTOR_M is from 0 to 3"]
    #[inline(always)]
    pub fn pll_m(&self) -> PLL_M_R {
        PLL_M_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PLL_EN_W {
        PLL_EN_W { w: self }
    }
    #[doc = "Bit 30 - LDO Enable"]
    #[inline(always)]
    pub fn pll_ldo_en(&mut self) -> PLL_LDO_EN_W {
        PLL_LDO_EN_W { w: self }
    }
    #[doc = "Bit 29 - Lock Enable"]
    #[inline(always)]
    pub fn lock_en(&mut self) -> LOCK_EN_W {
        LOCK_EN_W { w: self }
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    pub fn pll_output_gate(&mut self) -> PLL_OUTPUT_GATE_W {
        PLL_OUTPUT_GATE_W { w: self }
    }
    #[doc = "Bits 24:26 - PLL Lock Time\n\nThe bit indicates the step amplitude from one frequency to another."]
    #[inline(always)]
    pub fn pll_lock_time(&mut self) -> PLL_LOCK_TIME_W {
        PLL_LOCK_TIME_W { w: self }
    }
    #[doc = "Bits 8:15 - PLL N\n\nN = PLL_N + 1\n\nPLL_N is from 0 to 254"]
    #[inline(always)]
    pub fn pll_n(&mut self) -> PLL_N_W {
        PLL_N_W { w: self }
    }
    #[doc = "Bits 6:7 - PLL Unlock Level"]
    #[inline(always)]
    pub fn pll_unlock_mdsel(&mut self) -> PLL_UNLOCK_MDSEL_W {
        PLL_UNLOCK_MDSEL_W { w: self }
    }
    #[doc = "Bit 5 - PLL Lock Level"]
    #[inline(always)]
    pub fn pll_lock_mdsel(&mut self) -> PLL_LOCK_MDSEL_W {
        PLL_LOCK_MDSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - PLL M\n\nM = PLL_FACTOR_M + 1\n\nPLL_FACTOR_M is from 0 to 3"]
    #[inline(always)]
    pub fn pll_m(&mut self) -> PLL_M_W {
        PLL_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_CPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_cpu_ctrl](index.html) module"]
pub struct PLL_CPU_CTRL_SPEC;
impl crate::RegisterSpec for PLL_CPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_cpu_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_CPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_cpu_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_CPU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_CPU_CTRL to value 0"]
impl crate::Resettable for PLL_CPU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
