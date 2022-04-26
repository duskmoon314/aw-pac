#[doc = "Register `PLL_VE_CTRL` reader"]
pub struct R(crate::R<PLL_VE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_VE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_VE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_VE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_VE_CTRL` writer"]
pub struct W(crate::W<PLL_VE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_VE_CTRL_SPEC>;
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
impl From<crate::W<PLL_VE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_VE_CTRL_SPEC>) -> Self {
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
pub struct PLL_EN_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
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
pub struct PLL_LDO_EN_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Lock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LOCK_ENABLE` reader - Lock Enable"]
pub struct LOCK_ENABLE_R(crate::FieldReader<bool>);
impl LOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LOCK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == LOCK_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for LOCK_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_ENABLE` writer - Lock Enable"]
pub struct LOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "PLL Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LOCK` reader - PLL Lock Status"]
pub struct LOCK_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
pub struct PLL_OUTPUT_GATE_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "PLL SDM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PLL_SDM_EN` reader - PLL SDM Enable"]
pub struct PLL_SDM_EN_R(crate::FieldReader<bool>);
impl PLL_SDM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_SDM_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PLL_SDM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PLL_SDM_EN_A::ENABLE
    }
}
impl core::ops::Deref for PLL_SDM_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_SDM_EN` writer - PLL SDM Enable"]
pub struct PLL_SDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SDM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_SDM_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `PLL_N` reader - PLL N"]
pub struct PLL_N_R(crate::FieldReader<u8>);
impl PLL_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_N_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_N` writer - PLL N"]
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
#[doc = "Field `PLL_UNLOCK_MDSEL` reader - PLL Unlock Level"]
pub struct PLL_UNLOCK_MDSEL_R(crate::FieldReader<u8>);
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
    type Target = crate::FieldReader<u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "PLL Lock Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PLL_LOCK_MDSEL` reader - PLL Lock Level"]
pub struct PLL_LOCK_MDSEL_R(crate::FieldReader<bool>);
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
    type Target = crate::FieldReader<bool>;
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `PLL_INPUT_DIV2` reader - PLL Input Div M1"]
pub struct PLL_INPUT_DIV2_R(crate::FieldReader<bool>);
impl PLL_INPUT_DIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_INPUT_DIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_INPUT_DIV2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_INPUT_DIV2` writer - PLL Input Div M1"]
pub struct PLL_INPUT_DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_INPUT_DIV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `PLL_OUTPUT_DIV2` reader - PLL Output Div M0"]
pub struct PLL_OUTPUT_DIV2_R(crate::FieldReader<bool>);
impl PLL_OUTPUT_DIV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_OUTPUT_DIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_OUTPUT_DIV2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_OUTPUT_DIV2` writer - PLL Output Div M0"]
pub struct PLL_OUTPUT_DIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_OUTPUT_DIV2_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&self) -> PLL_EN_R {
        PLL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - LDO Enable"]
    #[inline(always)]
    pub fn pll_ldo_en(&self) -> PLL_LDO_EN_R {
        PLL_LDO_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock Enable"]
    #[inline(always)]
    pub fn lock_enable(&self) -> LOCK_ENABLE_R {
        LOCK_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    pub fn pll_output_gate(&self) -> PLL_OUTPUT_GATE_R {
        PLL_OUTPUT_GATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL SDM Enable"]
    #[inline(always)]
    pub fn pll_sdm_en(&self) -> PLL_SDM_EN_R {
        PLL_SDM_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PLL N"]
    #[inline(always)]
    pub fn pll_n(&self) -> PLL_N_R {
        PLL_N_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - PLL Unlock Level"]
    #[inline(always)]
    pub fn pll_unlock_mdsel(&self) -> PLL_UNLOCK_MDSEL_R {
        PLL_UNLOCK_MDSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5 - PLL Lock Level"]
    #[inline(always)]
    pub fn pll_lock_mdsel(&self) -> PLL_LOCK_MDSEL_R {
        PLL_LOCK_MDSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 1 - PLL Input Div M1"]
    #[inline(always)]
    pub fn pll_input_div2(&self) -> PLL_INPUT_DIV2_R {
        PLL_INPUT_DIV2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - PLL Output Div M0"]
    #[inline(always)]
    pub fn pll_output_div2(&self) -> PLL_OUTPUT_DIV2_R {
        PLL_OUTPUT_DIV2_R::new((self.bits & 1) != 0)
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
    pub fn lock_enable(&mut self) -> LOCK_ENABLE_W {
        LOCK_ENABLE_W { w: self }
    }
    #[doc = "Bit 27 - PLL Output Gating Enable"]
    #[inline(always)]
    pub fn pll_output_gate(&mut self) -> PLL_OUTPUT_GATE_W {
        PLL_OUTPUT_GATE_W { w: self }
    }
    #[doc = "Bit 24 - PLL SDM Enable"]
    #[inline(always)]
    pub fn pll_sdm_en(&mut self) -> PLL_SDM_EN_W {
        PLL_SDM_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - PLL N"]
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
    #[doc = "Bit 1 - PLL Input Div M1"]
    #[inline(always)]
    pub fn pll_input_div2(&mut self) -> PLL_INPUT_DIV2_W {
        PLL_INPUT_DIV2_W { w: self }
    }
    #[doc = "Bit 0 - PLL Output Div M0"]
    #[inline(always)]
    pub fn pll_output_div2(&mut self) -> PLL_OUTPUT_DIV2_W {
        PLL_OUTPUT_DIV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL_VE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_ve_ctrl](index.html) module"]
pub struct PLL_VE_CTRL_SPEC;
impl crate::RegisterSpec for PLL_VE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_ve_ctrl::R](R) reader structure"]
impl crate::Readable for PLL_VE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_ve_ctrl::W](W) writer structure"]
impl crate::Writable for PLL_VE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_VE_CTRL to value 0"]
impl crate::Resettable for PLL_VE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
