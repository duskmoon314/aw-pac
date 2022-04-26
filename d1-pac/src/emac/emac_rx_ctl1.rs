#[doc = "Register `EMAC_RX_CTL1` reader"]
pub struct R(crate::R<EMAC_RX_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RX_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RX_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RX_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_RX_CTL1` writer"]
pub struct W(crate::W<EMAC_RX_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RX_CTL1_SPEC>;
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
impl From<crate::W<EMAC_RX_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RX_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DMA_START` reader - "]
pub struct RX_DMA_START_R(crate::FieldReader<bool>);
impl RX_DMA_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DMA_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DMA_START_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DMA_START` writer - "]
pub struct RX_DMA_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA_START_W<'a> {
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
#[doc = "Receive DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EMA_EN_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<RX_EMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EMA_EN` reader - Receive DMA Enable"]
pub struct RX_EMA_EN_R(crate::FieldReader<bool>);
impl RX_EMA_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EMA_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMA_EN_A {
        match self.bits {
            false => RX_EMA_EN_A::STOP,
            true => RX_EMA_EN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == RX_EMA_EN_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == RX_EMA_EN_A::START
    }
}
impl core::ops::Deref for RX_EMA_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EMA_EN` writer - Receive DMA Enable"]
pub struct RX_EMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_EMA_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RX_EMA_EN_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RX_EMA_EN_A::START)
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
#[doc = "Receive FIFO Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_FLOW_CTL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_FIFO_FLOW_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_FLOW_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_FLOW_CTL` reader - Receive FIFO Flow Control Enable"]
pub struct RX_FIFO_FLOW_CTL_R(crate::FieldReader<bool>);
impl RX_FIFO_FLOW_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_FLOW_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_FLOW_CTL_A {
        match self.bits {
            false => RX_FIFO_FLOW_CTL_A::DISABLE,
            true => RX_FIFO_FLOW_CTL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_FIFO_FLOW_CTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_FIFO_FLOW_CTL_A::ENABLE
    }
}
impl core::ops::Deref for RX_FIFO_FLOW_CTL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FIFO_FLOW_CTL` writer - Receive FIFO Flow Control Enable"]
pub struct RX_FIFO_FLOW_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_FLOW_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_FLOW_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_FIFO_FLOW_CTL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_FIFO_FLOW_CTL_A::ENABLE)
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
#[doc = "Threshold for Deactivating Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FLOW_CTL_TH_DEACT_A {
    #[doc = "0: `0`"]
    FM1K = 0,
    #[doc = "1: `1`"]
    FM2K = 1,
    #[doc = "2: `10`"]
    FM3K = 2,
    #[doc = "3: `11`"]
    FM4K = 3,
}
impl From<RX_FLOW_CTL_TH_DEACT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FLOW_CTL_TH_DEACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_FLOW_CTL_TH_DEACT` reader - Threshold for Deactivating Flow Control"]
pub struct RX_FLOW_CTL_TH_DEACT_R(crate::FieldReader<u8>);
impl RX_FLOW_CTL_TH_DEACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FLOW_CTL_TH_DEACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FLOW_CTL_TH_DEACT_A {
        match self.bits {
            0 => RX_FLOW_CTL_TH_DEACT_A::FM1K,
            1 => RX_FLOW_CTL_TH_DEACT_A::FM2K,
            2 => RX_FLOW_CTL_TH_DEACT_A::FM3K,
            3 => RX_FLOW_CTL_TH_DEACT_A::FM4K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FM1K`"]
    #[inline(always)]
    pub fn is_fm1k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_DEACT_A::FM1K
    }
    #[doc = "Checks if the value of the field is `FM2K`"]
    #[inline(always)]
    pub fn is_fm2k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_DEACT_A::FM2K
    }
    #[doc = "Checks if the value of the field is `FM3K`"]
    #[inline(always)]
    pub fn is_fm3k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_DEACT_A::FM3K
    }
    #[doc = "Checks if the value of the field is `FM4K`"]
    #[inline(always)]
    pub fn is_fm4k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_DEACT_A::FM4K
    }
}
impl core::ops::Deref for RX_FLOW_CTL_TH_DEACT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_CTL_TH_DEACT` writer - Threshold for Deactivating Flow Control"]
pub struct RX_FLOW_CTL_TH_DEACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_CTL_TH_DEACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FLOW_CTL_TH_DEACT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fm1k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM1K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fm2k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM2K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fm3k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM3K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fm4k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM4K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Threshold for Activating Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FLOW_CTL_TH_ACT_A {
    #[doc = "0: `0`"]
    FM1K = 0,
    #[doc = "1: `1`"]
    FM2K = 1,
    #[doc = "2: `10`"]
    FM3K = 2,
    #[doc = "3: `11`"]
    FM4K = 3,
}
impl From<RX_FLOW_CTL_TH_ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FLOW_CTL_TH_ACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_FLOW_CTL_TH_ACT` reader - Threshold for Activating Flow Control"]
pub struct RX_FLOW_CTL_TH_ACT_R(crate::FieldReader<u8>);
impl RX_FLOW_CTL_TH_ACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FLOW_CTL_TH_ACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FLOW_CTL_TH_ACT_A {
        match self.bits {
            0 => RX_FLOW_CTL_TH_ACT_A::FM1K,
            1 => RX_FLOW_CTL_TH_ACT_A::FM2K,
            2 => RX_FLOW_CTL_TH_ACT_A::FM3K,
            3 => RX_FLOW_CTL_TH_ACT_A::FM4K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FM1K`"]
    #[inline(always)]
    pub fn is_fm1k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_ACT_A::FM1K
    }
    #[doc = "Checks if the value of the field is `FM2K`"]
    #[inline(always)]
    pub fn is_fm2k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_ACT_A::FM2K
    }
    #[doc = "Checks if the value of the field is `FM3K`"]
    #[inline(always)]
    pub fn is_fm3k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_ACT_A::FM3K
    }
    #[doc = "Checks if the value of the field is `FM4K`"]
    #[inline(always)]
    pub fn is_fm4k(&self) -> bool {
        **self == RX_FLOW_CTL_TH_ACT_A::FM4K
    }
}
impl core::ops::Deref for RX_FLOW_CTL_TH_ACT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_CTL_TH_ACT` writer - Threshold for Activating Flow Control"]
pub struct RX_FLOW_CTL_TH_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_CTL_TH_ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FLOW_CTL_TH_ACT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fm1k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM1K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fm2k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM2K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fm3k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM3K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fm4k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM4K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Threshold for RX DMA FIFO Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_TH_A {
    #[doc = "0: `0`"]
    T64 = 0,
    #[doc = "1: `1`"]
    T32 = 1,
    #[doc = "2: `10`"]
    T96 = 2,
    #[doc = "3: `11`"]
    T128 = 3,
}
impl From<RX_TH_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_TH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RX_TH` reader - Threshold for RX DMA FIFO Start"]
pub struct RX_TH_R(crate::FieldReader<u8>);
impl RX_TH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_TH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_TH_A {
        match self.bits {
            0 => RX_TH_A::T64,
            1 => RX_TH_A::T32,
            2 => RX_TH_A::T96,
            3 => RX_TH_A::T128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T64`"]
    #[inline(always)]
    pub fn is_t64(&self) -> bool {
        **self == RX_TH_A::T64
    }
    #[doc = "Checks if the value of the field is `T32`"]
    #[inline(always)]
    pub fn is_t32(&self) -> bool {
        **self == RX_TH_A::T32
    }
    #[doc = "Checks if the value of the field is `T96`"]
    #[inline(always)]
    pub fn is_t96(&self) -> bool {
        **self == RX_TH_A::T96
    }
    #[doc = "Checks if the value of the field is `T128`"]
    #[inline(always)]
    pub fn is_t128(&self) -> bool {
        **self == RX_TH_A::T128
    }
}
impl core::ops::Deref for RX_TH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TH` writer - Threshold for RX DMA FIFO Start"]
pub struct RX_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_TH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t64(self) -> &'a mut W {
        self.variant(RX_TH_A::T64)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t32(self) -> &'a mut W {
        self.variant(RX_TH_A::T32)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t96(self) -> &'a mut W {
        self.variant(RX_TH_A::T96)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t128(self) -> &'a mut W {
        self.variant(RX_TH_A::T128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_ERR_FRM_A {
    #[doc = "0: `0`"]
    DROP = 0,
    #[doc = "1: `1`"]
    FORWARD = 1,
}
impl From<RX_ERR_FRM_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ERR_FRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_ERR_FRM` reader - "]
pub struct RX_ERR_FRM_R(crate::FieldReader<bool>);
impl RX_ERR_FRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_ERR_FRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ERR_FRM_A {
        match self.bits {
            false => RX_ERR_FRM_A::DROP,
            true => RX_ERR_FRM_A::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        **self == RX_ERR_FRM_A::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        **self == RX_ERR_FRM_A::FORWARD
    }
}
impl core::ops::Deref for RX_ERR_FRM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_FRM` writer - "]
pub struct RX_ERR_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_FRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_ERR_FRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(RX_ERR_FRM_A::DROP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(RX_ERR_FRM_A::FORWARD)
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
#[doc = "Field `RX_RUNT_FRM` reader - "]
pub struct RX_RUNT_FRM_R(crate::FieldReader<bool>);
impl RX_RUNT_FRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_RUNT_FRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_RUNT_FRM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_RUNT_FRM` writer - "]
pub struct RX_RUNT_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RUNT_FRM_W<'a> {
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
#[doc = "Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_MD_A {
    #[doc = "0: `0`"]
    GREATER_THAN_TH = 0,
    #[doc = "1: `1`"]
    LOCATE_FULL_FRAME = 1,
}
impl From<RX_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_MD` reader - Receive Mode"]
pub struct RX_MD_R(crate::FieldReader<bool>);
impl RX_MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_MD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MD_A {
        match self.bits {
            false => RX_MD_A::GREATER_THAN_TH,
            true => RX_MD_A::LOCATE_FULL_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `GREATER_THAN_TH`"]
    #[inline(always)]
    pub fn is_greater_than_th(&self) -> bool {
        **self == RX_MD_A::GREATER_THAN_TH
    }
    #[doc = "Checks if the value of the field is `LOCATE_FULL_FRAME`"]
    #[inline(always)]
    pub fn is_locate_full_frame(&self) -> bool {
        **self == RX_MD_A::LOCATE_FULL_FRAME
    }
}
impl core::ops::Deref for RX_MD_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MD` writer - Receive Mode"]
pub struct RX_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_MD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn greater_than_th(self) -> &'a mut W {
        self.variant(RX_MD_A::GREATER_THAN_TH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locate_full_frame(self) -> &'a mut W {
        self.variant(RX_MD_A::LOCATE_FULL_FRAME)
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
#[doc = "Flush Receive Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLUSH_RX_FRM_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<FLUSH_RX_FRM_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_RX_FRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH_RX_FRM` reader - Flush Receive Frames"]
pub struct FLUSH_RX_FRM_R(crate::FieldReader<bool>);
impl FLUSH_RX_FRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSH_RX_FRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLUSH_RX_FRM_A {
        match self.bits {
            false => FLUSH_RX_FRM_A::ENABLE,
            true => FLUSH_RX_FRM_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FLUSH_RX_FRM_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FLUSH_RX_FRM_A::DISABLE
    }
}
impl core::ops::Deref for FLUSH_RX_FRM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSH_RX_FRM` writer - Flush Receive Frames"]
pub struct FLUSH_RX_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_RX_FRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLUSH_RX_FRM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLUSH_RX_FRM_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLUSH_RX_FRM_A::DISABLE)
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_dma_start(&self) -> RX_DMA_START_R {
        RX_DMA_START_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rx_ema_en(&self) -> RX_EMA_EN_R {
        RX_EMA_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive FIFO Flow Control Enable"]
    #[inline(always)]
    pub fn rx_fifo_flow_ctl(&self) -> RX_FIFO_FLOW_CTL_R {
        RX_FIFO_FLOW_CTL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_deact(&self) -> RX_FLOW_CTL_TH_DEACT_R {
        RX_FLOW_CTL_TH_DEACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_act(&self) -> RX_FLOW_CTL_TH_ACT_R {
        RX_FLOW_CTL_TH_ACT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Threshold for RX DMA FIFO Start"]
    #[inline(always)]
    pub fn rx_th(&self) -> RX_TH_R {
        RX_TH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_err_frm(&self) -> RX_ERR_FRM_R {
        RX_ERR_FRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_runt_frm(&self) -> RX_RUNT_FRM_R {
        RX_RUNT_FRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Mode"]
    #[inline(always)]
    pub fn rx_md(&self) -> RX_MD_R {
        RX_MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Flush Receive Frames"]
    #[inline(always)]
    pub fn flush_rx_frm(&self) -> FLUSH_RX_FRM_R {
        FLUSH_RX_FRM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_dma_start(&mut self) -> RX_DMA_START_W {
        RX_DMA_START_W { w: self }
    }
    #[doc = "Bit 30 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rx_ema_en(&mut self) -> RX_EMA_EN_W {
        RX_EMA_EN_W { w: self }
    }
    #[doc = "Bit 24 - Receive FIFO Flow Control Enable"]
    #[inline(always)]
    pub fn rx_fifo_flow_ctl(&mut self) -> RX_FIFO_FLOW_CTL_W {
        RX_FIFO_FLOW_CTL_W { w: self }
    }
    #[doc = "Bits 22:23 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_deact(&mut self) -> RX_FLOW_CTL_TH_DEACT_W {
        RX_FLOW_CTL_TH_DEACT_W { w: self }
    }
    #[doc = "Bits 20:21 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_act(&mut self) -> RX_FLOW_CTL_TH_ACT_W {
        RX_FLOW_CTL_TH_ACT_W { w: self }
    }
    #[doc = "Bits 4:5 - Threshold for RX DMA FIFO Start"]
    #[inline(always)]
    pub fn rx_th(&mut self) -> RX_TH_W {
        RX_TH_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_err_frm(&mut self) -> RX_ERR_FRM_W {
        RX_ERR_FRM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_runt_frm(&mut self) -> RX_RUNT_FRM_W {
        RX_RUNT_FRM_W { w: self }
    }
    #[doc = "Bit 1 - Receive Mode"]
    #[inline(always)]
    pub fn rx_md(&mut self) -> RX_MD_W {
        RX_MD_W { w: self }
    }
    #[doc = "Bit 0 - Flush Receive Frames"]
    #[inline(always)]
    pub fn flush_rx_frm(&mut self) -> FLUSH_RX_FRM_W {
        FLUSH_RX_FRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Receive Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rx_ctl1](index.html) module"]
pub struct EMAC_RX_CTL1_SPEC;
impl crate::RegisterSpec for EMAC_RX_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rx_ctl1::R](R) reader structure"]
impl crate::Readable for EMAC_RX_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rx_ctl1::W](W) writer structure"]
impl crate::Writable for EMAC_RX_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_RX_CTL1 to value 0"]
impl crate::Resettable for EMAC_RX_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
