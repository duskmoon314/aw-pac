#[doc = "Register `spi_batc` reader"]
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
#[doc = "Register `spi_batc` writer"]
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
#[doc = "Field `wms` reader - Work Mode Select"]
pub type WMS_R = crate::FieldReader<u8, WMS_A>;
#[doc = "Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WMS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == WMS_A::BYTE_ALIGNED
    }
    #[doc = "Checks if the value of the field is `BIT_ALIGNED_3WIRE`"]
    #[inline(always)]
    pub fn is_bit_aligned_3wire(&self) -> bool {
        *self == WMS_A::BIT_ALIGNED_3WIRE
    }
    #[doc = "Checks if the value of the field is `BIT_ALIGNED_STANDARD`"]
    #[inline(always)]
    pub fn is_bit_aligned_standard(&self) -> bool {
        *self == WMS_A::BIT_ALIGNED_STANDARD
    }
}
#[doc = "Field `wms` writer - Work Mode Select"]
pub type WMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BATC_SPEC, u8, WMS_A, 2, O>;
impl<'a, const O: u8> WMS_W<'a, O> {
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
}
#[doc = "Field `ss_sel` reader - SPI Chip Select"]
pub type SS_SEL_R = crate::FieldReader<u8, SS_SEL_A>;
#[doc = "SPI Chip Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SS_SEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SS_SEL_A::SS0
    }
    #[doc = "Checks if the value of the field is `SS1`"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == SS_SEL_A::SS1
    }
    #[doc = "Checks if the value of the field is `SS2`"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == SS_SEL_A::SS2
    }
    #[doc = "Checks if the value of the field is `SS3`"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == SS_SEL_A::SS3
    }
}
#[doc = "Field `ss_sel` writer - SPI Chip Select"]
pub type SS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI_BATC_SPEC, u8, SS_SEL_A, 2, O>;
impl<'a, const O: u8> SS_SEL_W<'a, O> {
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
}
#[doc = "Field `spol` reader - SPI Chip Select Signal Polarity Control"]
pub type SPOL_R = crate::BitReader<SPOL_A>;
#[doc = "SPI Chip Select Signal Polarity Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL_A::LOW
    }
}
#[doc = "Field `spol` writer - SPI Chip Select Signal Polarity Control"]
pub type SPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, SPOL_A, O>;
impl<'a, const O: u8> SPOL_W<'a, O> {
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
}
#[doc = "Field `ss_owner` reader - SS Output Owner Select"]
pub type SS_OWNER_R = crate::BitReader<SS_OWNER_A>;
#[doc = "SS Output Owner Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SS_OWNER_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SS_OWNER_A::SPI_CONTROLLER
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SS_OWNER_A::SOFTWARE
    }
}
#[doc = "Field `ss_owner` writer - SS Output Owner Select"]
pub type SS_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, SS_OWNER_A, O>;
impl<'a, const O: u8> SS_OWNER_W<'a, O> {
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
}
#[doc = "Field `ss_level` reader - "]
pub type SS_LEVEL_R = crate::BitReader<SS_LEVEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SS_LEVEL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == SS_LEVEL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SS_LEVEL_A::HIGH
    }
}
#[doc = "Field `ss_level` writer - "]
pub type SS_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, SS_LEVEL_A, O>;
impl<'a, const O: u8> SS_LEVEL_W<'a, O> {
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
}
#[doc = "Field `tx_frm_len` reader - Configure the length of serial data frame of TX"]
pub type TX_FRM_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_frm_len` writer - Configure the length of serial data frame of TX"]
pub type TX_FRM_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BATC_SPEC, u8, u8, 6, O>;
#[doc = "Field `rx_frm_len` reader - Configure the length of serial data frame of RX"]
pub type RX_FRM_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_frm_len` writer - Configure the length of serial data frame of RX"]
pub type RX_FRM_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_BATC_SPEC, u8, u8, 6, O>;
#[doc = "Field `tbc_int_en` reader - Transfer Bits Completed Interrupt Enable"]
pub type TBC_INT_EN_R = crate::BitReader<TBC_INT_EN_A>;
#[doc = "Transfer Bits Completed Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBC_INT_EN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TBC_INT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TBC_INT_EN_A::ENABLE
    }
}
#[doc = "Field `tbc_int_en` writer - Transfer Bits Completed Interrupt Enable"]
pub type TBC_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, TBC_INT_EN_A, O>;
impl<'a, const O: u8> TBC_INT_EN_W<'a, O> {
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
}
#[doc = "Field `tbc` reader - Transfer Bits Completed"]
pub type TBC_R = crate::BitReader<TBC_A>;
#[doc = "Transfer Bits Completed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TBC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TBC_A::BUSY
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TBC_A::COMPLETED
    }
}
#[doc = "Field `tbc` writer - Transfer Bits Completed"]
pub type TBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, TBC_A, O>;
impl<'a, const O: u8> TBC_W<'a, O> {
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
}
#[doc = "Field `msms` reader - Master Sample Standard"]
pub type MSMS_R = crate::BitReader<MSMS_A>;
#[doc = "Master Sample Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSMS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSMS_A::DELAY
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == MSMS_A::STANDARD
    }
}
#[doc = "Field `msms` writer - Master Sample Standard"]
pub type MSMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, MSMS_A, O>;
impl<'a, const O: u8> MSMS_W<'a, O> {
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
}
#[doc = "Field `tce` reader - Transfer Control Enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
#[doc = "Transfer Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TCE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TCE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `INIT`"]
    #[inline(always)]
    pub fn is_init(&self) -> bool {
        *self == TCE_A::INIT
    }
}
#[doc = "Field `tce` writer - Transfer Control Enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_BATC_SPEC, TCE_A, O>;
impl<'a, const O: u8> TCE_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Work Mode Select"]
    #[inline(always)]
    pub fn wms(&self) -> WMS_R {
        WMS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI Chip Select"]
    #[inline(always)]
    pub fn ss_sel(&self) -> SS_SEL_R {
        SS_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SS Output Owner Select"]
    #[inline(always)]
    pub fn ss_owner(&self) -> SS_OWNER_R {
        SS_OWNER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ss_level(&self) -> SS_LEVEL_R {
        SS_LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Configure the length of serial data frame of TX"]
    #[inline(always)]
    pub fn tx_frm_len(&self) -> TX_FRM_LEN_R {
        TX_FRM_LEN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Configure the length of serial data frame of RX"]
    #[inline(always)]
    pub fn rx_frm_len(&self) -> RX_FRM_LEN_R {
        RX_FRM_LEN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Transfer Bits Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tbc_int_en(&self) -> TBC_INT_EN_R {
        TBC_INT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transfer Bits Completed"]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Master Sample Standard"]
    #[inline(always)]
    pub fn msms(&self) -> MSMS_R {
        MSMS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transfer Control Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Work Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn wms(&mut self) -> WMS_W<0> {
        WMS_W::new(self)
    }
    #[doc = "Bits 2:3 - SPI Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn ss_sel(&mut self) -> SS_SEL_W<2> {
        SS_SEL_W::new(self)
    }
    #[doc = "Bit 5 - SPI Chip Select Signal Polarity Control"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<5> {
        SPOL_W::new(self)
    }
    #[doc = "Bit 6 - SS Output Owner Select"]
    #[inline(always)]
    #[must_use]
    pub fn ss_owner(&mut self) -> SS_OWNER_W<6> {
        SS_OWNER_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ss_level(&mut self) -> SS_LEVEL_W<7> {
        SS_LEVEL_W::new(self)
    }
    #[doc = "Bits 8:13 - Configure the length of serial data frame of TX"]
    #[inline(always)]
    #[must_use]
    pub fn tx_frm_len(&mut self) -> TX_FRM_LEN_W<8> {
        TX_FRM_LEN_W::new(self)
    }
    #[doc = "Bits 16:21 - Configure the length of serial data frame of RX"]
    #[inline(always)]
    #[must_use]
    pub fn rx_frm_len(&mut self) -> RX_FRM_LEN_W<16> {
        RX_FRM_LEN_W::new(self)
    }
    #[doc = "Bit 24 - Transfer Bits Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbc_int_en(&mut self) -> TBC_INT_EN_W<24> {
        TBC_INT_EN_W::new(self)
    }
    #[doc = "Bit 25 - Transfer Bits Completed"]
    #[inline(always)]
    #[must_use]
    pub fn tbc(&mut self) -> TBC_W<25> {
        TBC_W::new(self)
    }
    #[doc = "Bit 30 - Master Sample Standard"]
    #[inline(always)]
    #[must_use]
    pub fn msms(&mut self) -> MSMS_W<30> {
        MSMS_W::new(self)
    }
    #[doc = "Bit 31 - Transfer Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<31> {
        TCE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_batc to value 0"]
impl crate::Resettable for SPI_BATC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
