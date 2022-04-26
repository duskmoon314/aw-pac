#[doc = "Register `SPI_BATC` reader"]
pub struct R(crate::R<SPI_BATC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BATC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BATC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BATC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_BATC` writer"]
pub struct W(crate::W<SPI_BATC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BATC_SPEC>;
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
impl From<crate::W<SPI_BATC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BATC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE_A {
    #[doc = "0: `0`"]
    IDLE = 0,
    #[doc = "1: `1`"]
    INIT = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tce` reader - Transfer Control Enable"]
pub struct TCE_R(crate::FieldReader<bool>);
impl TCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::IDLE,
            true => TCE_A::INIT,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == TCE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `INIT`"]
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        **self == TCE_A::INIT
    }
}
impl core::ops::Deref for TCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tce` writer - Transfer Control Enable"]
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(TCE_A::IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn init(self) -> &'a mut W {
        self.variant(TCE_A::INIT)
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
#[doc = "Master Sample Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSMS_A {
    #[doc = "0: `0`"]
    DELAY = 0,
    #[doc = "1: `1`"]
    STANDARD = 1,
}
impl From<MSMS_A> for bool {
    #[inline(always)]
    fn from(variant: MSMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `msms` reader - Master Sample Standard"]
pub struct MSMS_R(crate::FieldReader<bool>);
impl MSMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSMS_A {
        match self.bits {
            false => MSMS_A::DELAY,
            true => MSMS_A::STANDARD,
        }
    }
    #[doc = "Checks if the value of the field is `DELAY`"]
    #[inline(always)]
    pub fn is_delay(&self) -> bool {
        **self == MSMS_A::DELAY
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == MSMS_A::STANDARD
    }
}
impl core::ops::Deref for MSMS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `msms` writer - Master Sample Standard"]
pub struct MSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSMS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn delay(self) -> &'a mut W {
        self.variant(MSMS_A::DELAY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(MSMS_A::STANDARD)
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
#[doc = "Transfer Bits Completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBC_A {
    #[doc = "0: `0`"]
    BUSY = 0,
    #[doc = "1: `1`"]
    COMPLETED = 1,
}
impl From<TBC_A> for bool {
    #[inline(always)]
    fn from(variant: TBC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tbc` reader - Transfer Bits Completed"]
pub struct TBC_R(crate::FieldReader<bool>);
impl TBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBC_A {
        match self.bits {
            false => TBC_A::BUSY,
            true => TBC_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == TBC_A::BUSY
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        **self == TBC_A::COMPLETED
    }
}
impl core::ops::Deref for TBC_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbc` writer - Transfer Bits Completed"]
pub struct TBC_W<'a> {
    w: &'a mut W,
}
impl<'a> TBC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(TBC_A::BUSY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(TBC_A::COMPLETED)
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Transfer Bits Completed Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBC_INT_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TBC_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TBC_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tbc_int_en` reader - Transfer Bits Completed Interrupt Enable"]
pub struct TBC_INT_EN_R(crate::FieldReader<bool>);
impl TBC_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBC_INT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBC_INT_EN_A {
        match self.bits {
            false => TBC_INT_EN_A::DISABLE,
            true => TBC_INT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TBC_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TBC_INT_EN_A::ENABLE
    }
}
impl core::ops::Deref for TBC_INT_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tbc_int_en` writer - Transfer Bits Completed Interrupt Enable"]
pub struct TBC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBC_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBC_INT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TBC_INT_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TBC_INT_EN_A::ENABLE)
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
#[doc = "Field `rx_frm_len` reader - Configure the length of serial data frame of RX"]
pub struct RX_FRM_LEN_R(crate::FieldReader<u8>);
impl RX_FRM_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FRM_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FRM_LEN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_frm_len` writer - Configure the length of serial data frame of RX"]
pub struct RX_FRM_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRM_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `tx_frm_len` reader - Configure the length of serial data frame of TX"]
pub struct TX_FRM_LEN_R(crate::FieldReader<u8>);
impl TX_FRM_LEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FRM_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FRM_LEN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_frm_len` writer - Configure the length of serial data frame of TX"]
pub struct TX_FRM_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FRM_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_LEVEL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SS_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: SS_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ss_level` reader - "]
pub struct SS_LEVEL_R(crate::FieldReader<bool>);
impl SS_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_LEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_LEVEL_A {
        match self.bits {
            false => SS_LEVEL_A::LOW,
            true => SS_LEVEL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SS_LEVEL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SS_LEVEL_A::HIGH
    }
}
impl core::ops::Deref for SS_LEVEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_level` writer - "]
pub struct SS_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_LEVEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SS_LEVEL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SS_LEVEL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "SS Output Owner Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_OWNER_A {
    #[doc = "0: `0`"]
    SPI_CONTROLLER = 0,
    #[doc = "1: `1`"]
    SOFTWARE = 1,
}
impl From<SS_OWNER_A> for bool {
    #[inline(always)]
    fn from(variant: SS_OWNER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ss_owner` reader - SS Output Owner Select"]
pub struct SS_OWNER_R(crate::FieldReader<bool>);
impl SS_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_OWNER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_OWNER_A {
        match self.bits {
            false => SS_OWNER_A::SPI_CONTROLLER,
            true => SS_OWNER_A::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_CONTROLLER`"]
    #[inline(always)]
    pub fn is_spi_controller(&self) -> bool {
        **self == SS_OWNER_A::SPI_CONTROLLER
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == SS_OWNER_A::SOFTWARE
    }
}
impl core::ops::Deref for SS_OWNER_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_owner` writer - SS Output Owner Select"]
pub struct SS_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_OWNER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_OWNER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn spi_controller(self) -> &'a mut W {
        self.variant(SS_OWNER_A::SPI_CONTROLLER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(SS_OWNER_A::SOFTWARE)
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "SPI Chip Select Signal Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL_A {
    #[doc = "0: `0`"]
    HIGH = 0,
    #[doc = "1: `1`"]
    LOW = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `spol` reader - SPI Chip Select Signal Polarity Control"]
pub struct SPOL_R(crate::FieldReader<bool>);
impl SPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::HIGH,
            true => SPOL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SPOL_A::LOW
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spol` writer - SPI Chip Select Signal Polarity Control"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL_A::HIGH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL_A::LOW)
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
#[doc = "SPI Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SS_SEL_A {
    #[doc = "0: `0`"]
    SS0 = 0,
    #[doc = "1: `1`"]
    SS1 = 1,
    #[doc = "2: `10`"]
    SS2 = 2,
    #[doc = "3: `11`"]
    SS3 = 3,
}
impl From<SS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ss_sel` reader - SPI Chip Select"]
pub struct SS_SEL_R(crate::FieldReader<u8>);
impl SS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_SEL_A {
        match self.bits {
            0 => SS_SEL_A::SS0,
            1 => SS_SEL_A::SS1,
            2 => SS_SEL_A::SS2,
            3 => SS_SEL_A::SS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SS0`"]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        **self == SS_SEL_A::SS0
    }
    #[doc = "Checks if the value of the field is `SS1`"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        **self == SS_SEL_A::SS1
    }
    #[doc = "Checks if the value of the field is `SS2`"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        **self == SS_SEL_A::SS2
    }
    #[doc = "Checks if the value of the field is `SS3`"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        **self == SS_SEL_A::SS3
    }
}
impl core::ops::Deref for SS_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_sel` writer - SPI Chip Select"]
pub struct SS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut W {
        self.variant(SS_SEL_A::SS0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut W {
        self.variant(SS_SEL_A::SS1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut W {
        self.variant(SS_SEL_A::SS2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut W {
        self.variant(SS_SEL_A::SS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WMS_A {
    #[doc = "0: `0`"]
    BYTE_ALIGNED = 0,
    #[doc = "2: `10`"]
    BIT_ALIGNED_3WIRE = 2,
    #[doc = "3: `11`"]
    BIT_ALIGNED_STANDARD = 3,
}
impl From<WMS_A> for u8 {
    #[inline(always)]
    fn from(variant: WMS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `wms` reader - Work Mode Select"]
pub struct WMS_R(crate::FieldReader<u8>);
impl WMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMS_A {
        match self.bits {
            0 => WMS_A::BYTE_ALIGNED,
            2 => WMS_A::BIT_ALIGNED_3WIRE,
            3 => WMS_A::BIT_ALIGNED_STANDARD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE_ALIGNED`"]
    #[inline(always)]
    pub fn is_byte_aligned(&self) -> bool {
        **self == WMS_A::BYTE_ALIGNED
    }
    #[doc = "Checks if the value of the field is `BIT_ALIGNED_3WIRE`"]
    #[inline(always)]
    pub fn is_bit_aligned_3wire(&self) -> bool {
        **self == WMS_A::BIT_ALIGNED_3WIRE
    }
    #[doc = "Checks if the value of the field is `BIT_ALIGNED_STANDARD`"]
    #[inline(always)]
    pub fn is_bit_aligned_standard(&self) -> bool {
        **self == WMS_A::BIT_ALIGNED_STANDARD
    }
}
impl core::ops::Deref for WMS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wms` writer - Work Mode Select"]
pub struct WMS_W<'a> {
    w: &'a mut W,
}
impl<'a> WMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn byte_aligned(self) -> &'a mut W {
        self.variant(WMS_A::BYTE_ALIGNED)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bit_aligned_3wire(self) -> &'a mut W {
        self.variant(WMS_A::BIT_ALIGNED_3WIRE)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn bit_aligned_standard(self) -> &'a mut W {
        self.variant(WMS_A::BIT_ALIGNED_STANDARD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Transfer Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Master Sample Standard"]
    #[inline(always)]
    pub fn msms(&self) -> MSMS_R {
        MSMS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 25 - Transfer Bits Completed"]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Transfer Bits Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tbc_int_en(&self) -> TBC_INT_EN_R {
        TBC_INT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Configure the length of serial data frame of RX"]
    #[inline(always)]
    pub fn rx_frm_len(&self) -> RX_FRM_LEN_R {
        RX_FRM_LEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Configure the length of serial data frame of TX"]
    #[inline(always)]
    pub fn tx_frm_len(&self) -> TX_FRM_LEN_R {
        TX_FRM_LEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ss_level(&self) -> SS_LEVEL_R {
        SS_LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - SS Output Owner Select"]
    #[inline(always)]
    pub fn ss_owner(&self) -> SS_OWNER_R {
        SS_OWNER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SPI Chip Select"]
    #[inline(always)]
    pub fn ss_sel(&self) -> SS_SEL_R {
        SS_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Work Mode Select"]
    #[inline(always)]
    pub fn wms(&self) -> WMS_R {
        WMS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Transfer Control Enable"]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    #[doc = "Bit 30 - Master Sample Standard"]
    #[inline(always)]
    pub fn msms(&mut self) -> MSMS_W {
        MSMS_W { w: self }
    }
    #[doc = "Bit 25 - Transfer Bits Completed"]
    #[inline(always)]
    pub fn tbc(&mut self) -> TBC_W {
        TBC_W { w: self }
    }
    #[doc = "Bit 24 - Transfer Bits Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tbc_int_en(&mut self) -> TBC_INT_EN_W {
        TBC_INT_EN_W { w: self }
    }
    #[doc = "Bits 16:21 - Configure the length of serial data frame of RX"]
    #[inline(always)]
    pub fn rx_frm_len(&mut self) -> RX_FRM_LEN_W {
        RX_FRM_LEN_W { w: self }
    }
    #[doc = "Bits 8:13 - Configure the length of serial data frame of TX"]
    #[inline(always)]
    pub fn tx_frm_len(&mut self) -> TX_FRM_LEN_W {
        TX_FRM_LEN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ss_level(&mut self) -> SS_LEVEL_W {
        SS_LEVEL_W { w: self }
    }
    #[doc = "Bit 6 - SS Output Owner Select"]
    #[inline(always)]
    pub fn ss_owner(&mut self) -> SS_OWNER_W {
        SS_OWNER_W { w: self }
    }
    #[doc = "Bit 5 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI Chip Select"]
    #[inline(always)]
    pub fn ss_sel(&mut self) -> SS_SEL_W {
        SS_SEL_W { w: self }
    }
    #[doc = "Bits 0:1 - Work Mode Select"]
    #[inline(always)]
    pub fn wms(&mut self) -> WMS_W {
        WMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Bit-Aligned Transfer Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_batc](index.html) module"]
pub struct SPI_BATC_SPEC;
impl crate::RegisterSpec for SPI_BATC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_batc::R](R) reader structure"]
impl crate::Readable for SPI_BATC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_batc::W](W) writer structure"]
impl crate::Writable for SPI_BATC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_BATC to value 0"]
impl crate::Resettable for SPI_BATC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
