#[doc = "Register `smhc_ctrl` reader"]
pub struct R(crate::R<SMHC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `smhc_ctrl` writer"]
pub struct W(crate::W<SMHC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CTRL_SPEC>;
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
impl From<crate::W<SMHC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `soft_rst` reader - Software Reset"]
pub type SOFT_RST_R = crate::BitReader<SOFT_RST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFT_RST_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_RST_A {
        match self.bits {
            false => SOFT_RST_A::NO_EFFECT,
            true => SOFT_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SOFT_RST_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RST_A::RESET
    }
}
#[doc = "Field `soft_rst` writer - Software Reset"]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, SOFT_RST_A, O>;
impl<'a, const O: u8> SOFT_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SOFT_RST_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFT_RST_A::RESET)
    }
}
#[doc = "Field `fifo_rst` reader - FIFO Reset"]
pub type FIFO_RST_R = crate::BitReader<FIFO_RST_A>;
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_RST_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<FIFO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_RST_A {
        match self.bits {
            false => FIFO_RST_A::NO_EFFECT,
            true => FIFO_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FIFO_RST_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FIFO_RST_A::RESET
    }
}
#[doc = "Field `fifo_rst` writer - FIFO Reset"]
pub type FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, FIFO_RST_A, O>;
impl<'a, const O: u8> FIFO_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FIFO_RST_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FIFO_RST_A::RESET)
    }
}
#[doc = "Field `dma_rst` reader - DMA Reset"]
pub type DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `dma_rst` writer - DMA Reset"]
pub type DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, bool, O>;
#[doc = "Field `ine_enb` reader - GLobal Interrupt Enable"]
pub type INE_ENB_R = crate::BitReader<INE_ENB_A>;
#[doc = "GLobal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INE_ENB_A {
    #[doc = "0: Disable interrupts"]
    DISABLE = 0,
    #[doc = "1: Enable interrupts"]
    ENABLE = 1,
}
impl From<INE_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: INE_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl INE_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INE_ENB_A {
        match self.bits {
            false => INE_ENB_A::DISABLE,
            true => INE_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INE_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INE_ENB_A::ENABLE
    }
}
#[doc = "Field `ine_enb` writer - GLobal Interrupt Enable"]
pub type INE_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, INE_ENB_A, O>;
impl<'a, const O: u8> INE_ENB_W<'a, O> {
    #[doc = "Disable interrupts"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INE_ENB_A::DISABLE)
    }
    #[doc = "Enable interrupts"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INE_ENB_A::ENABLE)
    }
}
#[doc = "Field `dma_enb` reader - DMA Global Enable"]
pub type DMA_ENB_R = crate::BitReader<DMA_ENB_A>;
#[doc = "DMA Global Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_ENB_A {
    #[doc = "0: Disable DMA to transfer data via AHB bus"]
    DISABLE = 0,
    #[doc = "1: Enable DMA to transfer data"]
    ENABLE = 1,
}
impl From<DMA_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENB_A {
        match self.bits {
            false => DMA_ENB_A::DISABLE,
            true => DMA_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_ENB_A::ENABLE
    }
}
#[doc = "Field `dma_enb` writer - DMA Global Enable"]
pub type DMA_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, DMA_ENB_A, O>;
impl<'a, const O: u8> DMA_ENB_W<'a, O> {
    #[doc = "Disable DMA to transfer data via AHB bus"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENB_A::DISABLE)
    }
    #[doc = "Enable DMA to transfer data"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENB_A::ENABLE)
    }
}
#[doc = "Field `cd_dbc_enb` reader - Card Detect (Data\\[3\\] status) De-bounce Enable"]
pub type CD_DBC_ENB_R = crate::BitReader<CD_DBC_ENB_A>;
#[doc = "Card Detect (Data\\[3\\] status) De-bounce Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CD_DBC_ENB_A {
    #[doc = "0: Disable de-bounce"]
    DISABLE = 0,
    #[doc = "1: Enable de-bounce"]
    ENABLE = 1,
}
impl From<CD_DBC_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CD_DBC_ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl CD_DBC_ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CD_DBC_ENB_A {
        match self.bits {
            false => CD_DBC_ENB_A::DISABLE,
            true => CD_DBC_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CD_DBC_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CD_DBC_ENB_A::ENABLE
    }
}
#[doc = "Field `cd_dbc_enb` writer - Card Detect (Data\\[3\\] status) De-bounce Enable"]
pub type CD_DBC_ENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, CD_DBC_ENB_A, O>;
impl<'a, const O: u8> CD_DBC_ENB_W<'a, O> {
    #[doc = "Disable de-bounce"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CD_DBC_ENB_A::DISABLE)
    }
    #[doc = "Enable de-bounce"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CD_DBC_ENB_A::ENABLE)
    }
}
#[doc = "Field `ddr_mod_sel` reader - DDR Mode Select"]
pub type DDR_MOD_SEL_R = crate::BitReader<DDR_MOD_SEL_A>;
#[doc = "DDR Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDR_MOD_SEL_A {
    #[doc = "0: SDR mode"]
    SDR = 0,
    #[doc = "1: DDR mode"]
    DDR = 1,
}
impl From<DDR_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DDR_MOD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR_MOD_SEL_A {
        match self.bits {
            false => DDR_MOD_SEL_A::SDR,
            true => DDR_MOD_SEL_A::DDR,
        }
    }
    #[doc = "Checks if the value of the field is `SDR`"]
    #[inline(always)]
    pub fn is_sdr(&self) -> bool {
        *self == DDR_MOD_SEL_A::SDR
    }
    #[doc = "Checks if the value of the field is `DDR`"]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        *self == DDR_MOD_SEL_A::DDR
    }
}
#[doc = "Field `ddr_mod_sel` writer - DDR Mode Select"]
pub type DDR_MOD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, DDR_MOD_SEL_A, O>;
impl<'a, const O: u8> DDR_MOD_SEL_W<'a, O> {
    #[doc = "SDR mode"]
    #[inline(always)]
    pub fn sdr(self) -> &'a mut W {
        self.variant(DDR_MOD_SEL_A::SDR)
    }
    #[doc = "DDR mode"]
    #[inline(always)]
    pub fn ddr(self) -> &'a mut W {
        self.variant(DDR_MOD_SEL_A::DDR)
    }
}
#[doc = "Field `time_unit_dat` reader - Time unit for data line"]
pub type TIME_UNIT_DAT_R = crate::BitReader<TIME_UNIT_DAT_A>;
#[doc = "Time unit for data line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIME_UNIT_DAT_A {
    #[doc = "0: 1 card clock period"]
    C1 = 0,
    #[doc = "1: 256 card clock period"]
    C256 = 1,
}
impl From<TIME_UNIT_DAT_A> for bool {
    #[inline(always)]
    fn from(variant: TIME_UNIT_DAT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIME_UNIT_DAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIME_UNIT_DAT_A {
        match self.bits {
            false => TIME_UNIT_DAT_A::C1,
            true => TIME_UNIT_DAT_A::C256,
        }
    }
    #[doc = "Checks if the value of the field is `C1`"]
    #[inline(always)]
    pub fn is_c1(&self) -> bool {
        *self == TIME_UNIT_DAT_A::C1
    }
    #[doc = "Checks if the value of the field is `C256`"]
    #[inline(always)]
    pub fn is_c256(&self) -> bool {
        *self == TIME_UNIT_DAT_A::C256
    }
}
#[doc = "Field `time_unit_dat` writer - Time unit for data line"]
pub type TIME_UNIT_DAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, TIME_UNIT_DAT_A, O>;
impl<'a, const O: u8> TIME_UNIT_DAT_W<'a, O> {
    #[doc = "1 card clock period"]
    #[inline(always)]
    pub fn c1(self) -> &'a mut W {
        self.variant(TIME_UNIT_DAT_A::C1)
    }
    #[doc = "256 card clock period"]
    #[inline(always)]
    pub fn c256(self) -> &'a mut W {
        self.variant(TIME_UNIT_DAT_A::C256)
    }
}
#[doc = "Field `time_unit_cmd` reader - Time unit for command line"]
pub type TIME_UNIT_CMD_R = crate::BitReader<TIME_UNIT_CMD_A>;
#[doc = "Time unit for command line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIME_UNIT_CMD_A {
    #[doc = "0: 1 card clock period"]
    C1 = 0,
    #[doc = "1: 256 card clock period"]
    C256 = 1,
}
impl From<TIME_UNIT_CMD_A> for bool {
    #[inline(always)]
    fn from(variant: TIME_UNIT_CMD_A) -> Self {
        variant as u8 != 0
    }
}
impl TIME_UNIT_CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIME_UNIT_CMD_A {
        match self.bits {
            false => TIME_UNIT_CMD_A::C1,
            true => TIME_UNIT_CMD_A::C256,
        }
    }
    #[doc = "Checks if the value of the field is `C1`"]
    #[inline(always)]
    pub fn is_c1(&self) -> bool {
        *self == TIME_UNIT_CMD_A::C1
    }
    #[doc = "Checks if the value of the field is `C256`"]
    #[inline(always)]
    pub fn is_c256(&self) -> bool {
        *self == TIME_UNIT_CMD_A::C256
    }
}
#[doc = "Field `time_unit_cmd` writer - Time unit for command line"]
pub type TIME_UNIT_CMD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, TIME_UNIT_CMD_A, O>;
impl<'a, const O: u8> TIME_UNIT_CMD_W<'a, O> {
    #[doc = "1 card clock period"]
    #[inline(always)]
    pub fn c1(self) -> &'a mut W {
        self.variant(TIME_UNIT_CMD_A::C1)
    }
    #[doc = "256 card clock period"]
    #[inline(always)]
    pub fn c256(self) -> &'a mut W {
        self.variant(TIME_UNIT_CMD_A::C256)
    }
}
#[doc = "Field `fifo_ac_mod` reader - FIFO Accesss Mode"]
pub type FIFO_AC_MOD_R = crate::BitReader<FIFO_AC_MOD_A>;
#[doc = "FIFO Accesss Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_AC_MOD_A {
    #[doc = "0: DMA bus"]
    DMA = 0,
    #[doc = "1: AHB bus"]
    AHB = 1,
}
impl From<FIFO_AC_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_AC_MOD_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_AC_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_AC_MOD_A {
        match self.bits {
            false => FIFO_AC_MOD_A::DMA,
            true => FIFO_AC_MOD_A::AHB,
        }
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == FIFO_AC_MOD_A::DMA
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == FIFO_AC_MOD_A::AHB
    }
}
#[doc = "Field `fifo_ac_mod` writer - FIFO Accesss Mode"]
pub type FIFO_AC_MOD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SMHC_CTRL_SPEC, FIFO_AC_MOD_A, O>;
impl<'a, const O: u8> FIFO_AC_MOD_W<'a, O> {
    #[doc = "DMA bus"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(FIFO_AC_MOD_A::DMA)
    }
    #[doc = "AHB bus"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut W {
        self.variant(FIFO_AC_MOD_A::AHB)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    pub fn fifo_rst(&self) -> FIFO_RST_R {
        FIFO_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Reset"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GLobal Interrupt Enable"]
    #[inline(always)]
    pub fn ine_enb(&self) -> INE_ENB_R {
        INE_ENB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Global Enable"]
    #[inline(always)]
    pub fn dma_enb(&self) -> DMA_ENB_R {
        DMA_ENB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Detect (Data\\[3\\] status) De-bounce Enable"]
    #[inline(always)]
    pub fn cd_dbc_enb(&self) -> CD_DBC_ENB_R {
        CD_DBC_ENB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - DDR Mode Select"]
    #[inline(always)]
    pub fn ddr_mod_sel(&self) -> DDR_MOD_SEL_R {
        DDR_MOD_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time unit for data line"]
    #[inline(always)]
    pub fn time_unit_dat(&self) -> TIME_UNIT_DAT_R {
        TIME_UNIT_DAT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time unit for command line"]
    #[inline(always)]
    pub fn time_unit_cmd(&self) -> TIME_UNIT_CMD_R {
        TIME_UNIT_CMD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - FIFO Accesss Mode"]
    #[inline(always)]
    pub fn fifo_ac_mod(&self) -> FIFO_AC_MOD_R {
        FIFO_AC_MOD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<0> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_rst(&mut self) -> FIFO_RST_W<1> {
        FIFO_RST_W::new(self)
    }
    #[doc = "Bit 2 - DMA Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst(&mut self) -> DMA_RST_W<2> {
        DMA_RST_W::new(self)
    }
    #[doc = "Bit 4 - GLobal Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ine_enb(&mut self) -> INE_ENB_W<4> {
        INE_ENB_W::new(self)
    }
    #[doc = "Bit 5 - DMA Global Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_enb(&mut self) -> DMA_ENB_W<5> {
        DMA_ENB_W::new(self)
    }
    #[doc = "Bit 8 - Card Detect (Data\\[3\\] status) De-bounce Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cd_dbc_enb(&mut self) -> CD_DBC_ENB_W<8> {
        CD_DBC_ENB_W::new(self)
    }
    #[doc = "Bit 10 - DDR Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_mod_sel(&mut self) -> DDR_MOD_SEL_W<10> {
        DDR_MOD_SEL_W::new(self)
    }
    #[doc = "Bit 11 - Time unit for data line"]
    #[inline(always)]
    #[must_use]
    pub fn time_unit_dat(&mut self) -> TIME_UNIT_DAT_W<11> {
        TIME_UNIT_DAT_W::new(self)
    }
    #[doc = "Bit 12 - Time unit for command line"]
    #[inline(always)]
    #[must_use]
    pub fn time_unit_cmd(&mut self) -> TIME_UNIT_CMD_W<12> {
        TIME_UNIT_CMD_W::new(self)
    }
    #[doc = "Bit 31 - FIFO Accesss Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_ac_mod(&mut self) -> FIFO_AC_MOD_W<31> {
        FIFO_AC_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ctrl](index.html) module"]
pub struct SMHC_CTRL_SPEC;
impl crate::RegisterSpec for SMHC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ctrl::R](R) reader structure"]
impl crate::Readable for SMHC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ctrl::W](W) writer structure"]
impl crate::Writable for SMHC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ctrl to value 0"]
impl crate::Resettable for SMHC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
