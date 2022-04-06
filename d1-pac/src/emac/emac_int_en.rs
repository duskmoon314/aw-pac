#[doc = "Register `EMAC_INT_EN` reader"]
pub struct R(crate::R<EMAC_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_INT_EN` writer"]
pub struct W(crate::W<EMAC_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_INT_EN_SPEC>;
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
impl From<crate::W<EMAC_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Early Receive Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EARLY_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_EARLY_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EARLY_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EARLY_INT_EN` reader - Early Receive Interrupt"]
pub struct RX_EARLY_INT_EN_R(crate::FieldReader<bool, RX_EARLY_INT_EN_A>);
impl RX_EARLY_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EARLY_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EARLY_INT_EN_A {
        match self.bits {
            false => RX_EARLY_INT_EN_A::DISABLE,
            true => RX_EARLY_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_EARLY_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_EARLY_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_EARLY_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_EARLY_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EARLY_INT_EN` writer - Early Receive Interrupt"]
pub struct RX_EARLY_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EARLY_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_EARLY_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_EARLY_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_EARLY_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Receive Overflow Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_OVERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_OVERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_OVERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OVERFLOW_INT_EN` reader - Receive Overflow Interrupt"]
pub struct RX_OVERFLOW_INT_EN_R(crate::FieldReader<bool, RX_OVERFLOW_INT_EN_A>);
impl RX_OVERFLOW_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_OVERFLOW_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_OVERFLOW_INT_EN_A {
        match self.bits {
            false => RX_OVERFLOW_INT_EN_A::DISABLE,
            true => RX_OVERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_OVERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_OVERFLOW_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_OVERFLOW_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_OVERFLOW_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_OVERFLOW_INT_EN` writer - Receive Overflow Interrupt"]
pub struct RX_OVERFLOW_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVERFLOW_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_OVERFLOW_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_OVERFLOW_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Receive Timeout Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_TIMEOUT_INT_EN` reader - Receive Timeout Interrupt"]
pub struct RX_TIMEOUT_INT_EN_R(crate::FieldReader<bool, RX_TIMEOUT_INT_EN_A>);
impl RX_TIMEOUT_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TIMEOUT_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_TIMEOUT_INT_EN_A {
        match self.bits {
            false => RX_TIMEOUT_INT_EN_A::DISABLE,
            true => RX_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_TIMEOUT_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_TIMEOUT_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_TIMEOUT_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TIMEOUT_INT_EN` writer - Receive Timeout Interrupt"]
pub struct RX_TIMEOUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TIMEOUT_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_TIMEOUT_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_TIMEOUT_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Receive DMA FSM Stopped Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DMA_STOPPED_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_DMA_STOPPED_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_DMA_STOPPED_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DMA_STOPPED_INT_EN` reader - Receive DMA FSM Stopped Interrupt"]
pub struct RX_DMA_STOPPED_INT_EN_R(crate::FieldReader<bool, RX_DMA_STOPPED_INT_EN_A>);
impl RX_DMA_STOPPED_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA_STOPPED_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_DMA_STOPPED_INT_EN_A {
        match self.bits {
            false => RX_DMA_STOPPED_INT_EN_A::DISABLE,
            true => RX_DMA_STOPPED_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_DMA_STOPPED_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_DMA_STOPPED_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_DMA_STOPPED_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_DMA_STOPPED_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA_STOPPED_INT_EN` writer - Receive DMA FSM Stopped Interrupt"]
pub struct RX_DMA_STOPPED_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_STOPPED_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_DMA_STOPPED_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_DMA_STOPPED_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_DMA_STOPPED_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Receive Buffer Unavailable Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_BUF_UA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_BUF_UA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_BUF_UA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_BUF_UA_INT_EN` reader - Receive Buffer Unavailable Interrupt"]
pub struct RX_BUF_UA_INT_EN_R(crate::FieldReader<bool, RX_BUF_UA_INT_EN_A>);
impl RX_BUF_UA_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BUF_UA_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_BUF_UA_INT_EN_A {
        match self.bits {
            false => RX_BUF_UA_INT_EN_A::DISABLE,
            true => RX_BUF_UA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_BUF_UA_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_BUF_UA_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_BUF_UA_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_BUF_UA_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BUF_UA_INT_EN` writer - Receive Buffer Unavailable Interrupt"]
pub struct RX_BUF_UA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_UA_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_BUF_UA_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_BUF_UA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_BUF_UA_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Receive Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_INT_EN` reader - Receive Interrupt"]
pub struct RX_INT_EN_R(crate::FieldReader<bool, RX_INT_EN_A>);
impl RX_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_INT_EN_A {
        match self.bits {
            false => RX_INT_EN_A::DISABLE,
            true => RX_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_INT_EN_R {
    type Target = crate::FieldReader<bool, RX_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_INT_EN` writer - Receive Interrupt"]
pub struct RX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Early Transmit Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_EARLY_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_EARLY_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_EARLY_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EARLY_INT_EN` reader - Early Transmit Interrupt"]
pub struct TX_EARLY_INT_EN_R(crate::FieldReader<bool, TX_EARLY_INT_EN_A>);
impl TX_EARLY_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_EARLY_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_EARLY_INT_EN_A {
        match self.bits {
            false => TX_EARLY_INT_EN_A::DISABLE,
            true => TX_EARLY_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_EARLY_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_EARLY_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_EARLY_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_EARLY_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_EARLY_INT_EN` writer - Early Transmit Interrupt"]
pub struct TX_EARLY_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EARLY_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_EARLY_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_EARLY_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_EARLY_INT_EN_A::ENABLE)
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
#[doc = "Transmit Underflow Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UNDERFLOW_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_UNDERFLOW_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_UNDERFLOW_INT_EN` reader - Transmit Underflow Interrupt"]
pub struct TX_UNDERFLOW_INT_EN_R(crate::FieldReader<bool, TX_UNDERFLOW_INT_EN_A>);
impl TX_UNDERFLOW_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_UNDERFLOW_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UNDERFLOW_INT_EN_A {
        match self.bits {
            false => TX_UNDERFLOW_INT_EN_A::DISABLE,
            true => TX_UNDERFLOW_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_UNDERFLOW_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_UNDERFLOW_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_UNDERFLOW_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_UNDERFLOW_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_UNDERFLOW_INT_EN` writer - Transmit Underflow Interrupt"]
pub struct TX_UNDERFLOW_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UNDERFLOW_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_UNDERFLOW_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Transmit Timeout Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_TIMEOUT_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_TIMEOUT_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_TIMEOUT_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_TIMEOUT_INT_EN` reader - Transmit Timeout Interrupt"]
pub struct TX_TIMEOUT_INT_EN_R(crate::FieldReader<bool, TX_TIMEOUT_INT_EN_A>);
impl TX_TIMEOUT_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_TIMEOUT_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_TIMEOUT_INT_EN_A {
        match self.bits {
            false => TX_TIMEOUT_INT_EN_A::DISABLE,
            true => TX_TIMEOUT_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_TIMEOUT_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_TIMEOUT_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_TIMEOUT_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_TIMEOUT_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TIMEOUT_INT_EN` writer - Transmit Timeout Interrupt"]
pub struct TX_TIMEOUT_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TIMEOUT_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_TIMEOUT_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_TIMEOUT_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_TIMEOUT_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Transmit Buffer Available Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_BUF_UA_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_BUF_UA_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_BUF_UA_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_BUF_UA_INT_EN` reader - Transmit Buffer Available Interrupt"]
pub struct TX_BUF_UA_INT_EN_R(crate::FieldReader<bool, TX_BUF_UA_INT_EN_A>);
impl TX_BUF_UA_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_BUF_UA_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_BUF_UA_INT_EN_A {
        match self.bits {
            false => TX_BUF_UA_INT_EN_A::DISABLE,
            true => TX_BUF_UA_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_BUF_UA_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_BUF_UA_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_BUF_UA_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_BUF_UA_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BUF_UA_INT_EN` writer - Transmit Buffer Available Interrupt"]
pub struct TX_BUF_UA_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUF_UA_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_BUF_UA_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_BUF_UA_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_BUF_UA_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Transmit DMA FSM Stopped Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DMA_STOPPED_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_DMA_STOPPED_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_DMA_STOPPED_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_DMA_STOPPED_INT_EN` reader - Transmit DMA FSM Stopped Interrupt"]
pub struct TX_DMA_STOPPED_INT_EN_R(crate::FieldReader<bool, TX_DMA_STOPPED_INT_EN_A>);
impl TX_DMA_STOPPED_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DMA_STOPPED_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_DMA_STOPPED_INT_EN_A {
        match self.bits {
            false => TX_DMA_STOPPED_INT_EN_A::DISABLE,
            true => TX_DMA_STOPPED_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_DMA_STOPPED_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_DMA_STOPPED_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_DMA_STOPPED_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_DMA_STOPPED_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DMA_STOPPED_INT_EN` writer - Transmit DMA FSM Stopped Interrupt"]
pub struct TX_DMA_STOPPED_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA_STOPPED_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_DMA_STOPPED_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_DMA_STOPPED_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_DMA_STOPPED_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Transmit Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_INT_EN` reader - Transmit Interrupt"]
pub struct TX_INT_EN_R(crate::FieldReader<bool, TX_INT_EN_A>);
impl TX_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_INT_EN_A {
        match self.bits {
            false => TX_INT_EN_A::DISABLE,
            true => TX_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TX_INT_EN_R {
    type Target = crate::FieldReader<bool, TX_INT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_INT_EN` writer - Transmit Interrupt"]
pub struct TX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_INT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn rx_early_int_en(&self) -> RX_EARLY_INT_EN_R {
        RX_EARLY_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Overflow Interrupt"]
    #[inline(always)]
    pub fn rx_overflow_int_en(&self) -> RX_OVERFLOW_INT_EN_R {
        RX_OVERFLOW_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Timeout Interrupt"]
    #[inline(always)]
    pub fn rx_timeout_int_en(&self) -> RX_TIMEOUT_INT_EN_R {
        RX_TIMEOUT_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn rx_dma_stopped_int_en(&self) -> RX_DMA_STOPPED_INT_EN_R {
        RX_DMA_STOPPED_INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Buffer Unavailable Interrupt"]
    #[inline(always)]
    pub fn rx_buf_ua_int_en(&self) -> RX_BUF_UA_INT_EN_R {
        RX_BUF_UA_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int_en(&self) -> RX_INT_EN_R {
        RX_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_early_int_en(&self) -> TX_EARLY_INT_EN_R {
        TX_EARLY_INT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Underflow Interrupt"]
    #[inline(always)]
    pub fn tx_underflow_int_en(&self) -> TX_UNDERFLOW_INT_EN_R {
        TX_UNDERFLOW_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Timeout Interrupt"]
    #[inline(always)]
    pub fn tx_timeout_int_en(&self) -> TX_TIMEOUT_INT_EN_R {
        TX_TIMEOUT_INT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Available Interrupt"]
    #[inline(always)]
    pub fn tx_buf_ua_int_en(&self) -> TX_BUF_UA_INT_EN_R {
        TX_BUF_UA_INT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn tx_dma_stopped_int_en(&self) -> TX_DMA_STOPPED_INT_EN_R {
        TX_DMA_STOPPED_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int_en(&self) -> TX_INT_EN_R {
        TX_INT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn rx_early_int_en(&mut self) -> RX_EARLY_INT_EN_W {
        RX_EARLY_INT_EN_W { w: self }
    }
    #[doc = "Bit 12 - Receive Overflow Interrupt"]
    #[inline(always)]
    pub fn rx_overflow_int_en(&mut self) -> RX_OVERFLOW_INT_EN_W {
        RX_OVERFLOW_INT_EN_W { w: self }
    }
    #[doc = "Bit 11 - Receive Timeout Interrupt"]
    #[inline(always)]
    pub fn rx_timeout_int_en(&mut self) -> RX_TIMEOUT_INT_EN_W {
        RX_TIMEOUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 10 - Receive DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn rx_dma_stopped_int_en(&mut self) -> RX_DMA_STOPPED_INT_EN_W {
        RX_DMA_STOPPED_INT_EN_W { w: self }
    }
    #[doc = "Bit 9 - Receive Buffer Unavailable Interrupt"]
    #[inline(always)]
    pub fn rx_buf_ua_int_en(&mut self) -> RX_BUF_UA_INT_EN_W {
        RX_BUF_UA_INT_EN_W { w: self }
    }
    #[doc = "Bit 8 - Receive Interrupt"]
    #[inline(always)]
    pub fn rx_int_en(&mut self) -> RX_INT_EN_W {
        RX_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_early_int_en(&mut self) -> TX_EARLY_INT_EN_W {
        TX_EARLY_INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Underflow Interrupt"]
    #[inline(always)]
    pub fn tx_underflow_int_en(&mut self) -> TX_UNDERFLOW_INT_EN_W {
        TX_UNDERFLOW_INT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Timeout Interrupt"]
    #[inline(always)]
    pub fn tx_timeout_int_en(&mut self) -> TX_TIMEOUT_INT_EN_W {
        TX_TIMEOUT_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Transmit Buffer Available Interrupt"]
    #[inline(always)]
    pub fn tx_buf_ua_int_en(&mut self) -> TX_BUF_UA_INT_EN_W {
        TX_BUF_UA_INT_EN_W { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA FSM Stopped Interrupt"]
    #[inline(always)]
    pub fn tx_dma_stopped_int_en(&mut self) -> TX_DMA_STOPPED_INT_EN_W {
        TX_DMA_STOPPED_INT_EN_W { w: self }
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn tx_int_en(&mut self) -> TX_INT_EN_W {
        TX_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_int_en](index.html) module"]
pub struct EMAC_INT_EN_SPEC;
impl crate::RegisterSpec for EMAC_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_int_en::R](R) reader structure"]
impl crate::Readable for EMAC_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_int_en::W](W) writer structure"]
impl crate::Writable for EMAC_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_INT_EN to value 0"]
impl crate::Resettable for EMAC_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
