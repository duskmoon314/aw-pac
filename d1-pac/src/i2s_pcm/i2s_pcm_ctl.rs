#[doc = "Register `i2s_pcm_ctl` reader"]
pub struct R(crate::R<I2S_PCM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_ctl` writer"]
pub struct W(crate::W<I2S_PCM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_CTL_SPEC>;
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
impl From<crate::W<I2S_PCM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gen` reader - Global Enable"]
pub type GEN_R = crate::BitReader<GEN_A>;
#[doc = "Global Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEN_A {
        match self.bits {
            false => GEN_A::DISABLE,
            true => GEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GEN_A::ENABLE
    }
}
#[doc = "Field `gen` writer - Global Enable"]
pub type GEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, GEN_A, O>;
impl<'a, const O: u8> GEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GEN_A::ENABLE)
    }
}
#[doc = "Field `rxen` reader - Receiver Block Enable"]
pub type RXEN_R = crate::BitReader<RXEN_A>;
#[doc = "Receiver Block Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLE,
            true => RXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXEN_A::ENABLE
    }
}
#[doc = "Field `rxen` writer - Receiver Block Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, RXEN_A, O>;
impl<'a, const O: u8> RXEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXEN_A::ENABLE)
    }
}
#[doc = "Field `txen` reader - Transmitter Block Enable"]
pub type TXEN_R = crate::BitReader<TXEN_A>;
#[doc = "Transmitter Block Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLE,
            true => TXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXEN_A::ENABLE
    }
}
#[doc = "Field `txen` writer - Transmitter Block Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, TXEN_A, O>;
impl<'a, const O: u8> TXEN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXEN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXEN_A::ENABLE)
    }
}
#[doc = "Field `loopback` reader - Loopback Test"]
pub type LOOPBACK_R = crate::BitReader<LOOPBACK_A>;
#[doc = "Loopback Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPBACK_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    TEST = 1,
}
impl From<LOOPBACK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPBACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPBACK_A {
        match self.bits {
            false => LOOPBACK_A::NORMAL,
            true => LOOPBACK_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LOOPBACK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == LOOPBACK_A::TEST
    }
}
#[doc = "Field `loopback` writer - Loopback Test"]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, LOOPBACK_A, O>;
impl<'a, const O: u8> LOOPBACK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LOOPBACK_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(LOOPBACK_A::TEST)
    }
}
#[doc = "Field `mode_sel` reader - Mode Selection"]
pub type MODE_SEL_R = crate::FieldReader<u8, MODE_SEL_A>;
#[doc = "Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_SEL_A {
    #[doc = "0: PCM Mode (offset 0: Long Frame, offset 1: Short Frame)"]
    PCM = 0,
    #[doc = "1: Left-justified Mode (offset 0: LJ Mode, offset 1: I2S Mode)"]
    LEFT = 1,
    #[doc = "2: Right-justified Mode"]
    RIGHT = 2,
}
impl From<MODE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_SEL_A) -> Self {
        variant as _
    }
}
impl MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SEL_A {
        match self.bits {
            0 => MODE_SEL_A::PCM,
            1 => MODE_SEL_A::LEFT,
            2 => MODE_SEL_A::RIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == MODE_SEL_A::PCM
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == MODE_SEL_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == MODE_SEL_A::RIGHT
    }
}
#[doc = "Field `mode_sel` writer - Mode Selection"]
pub type MODE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_CTL_SPEC, u8, MODE_SEL_A, 2, O>;
impl<'a, const O: u8> MODE_SEL_W<'a, O> {
    #[doc = "PCM Mode (offset 0: Long Frame, offset 1: Short Frame)"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(MODE_SEL_A::PCM)
    }
    #[doc = "Left-justified Mode (offset 0: LJ Mode, offset 1: I2S Mode)"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(MODE_SEL_A::LEFT)
    }
    #[doc = "Right-justified Mode"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(MODE_SEL_A::RIGHT)
    }
}
#[doc = "Field `out_mute` reader - Data Output Mute Enable"]
pub type OUT_MUTE_R = crate::BitReader<OUT_MUTE_A>;
#[doc = "Data Output Mute Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT_MUTE_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    ZERO = 1,
}
impl From<OUT_MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: OUT_MUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT_MUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_MUTE_A {
        match self.bits {
            false => OUT_MUTE_A::NORMAL,
            true => OUT_MUTE_A::ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OUT_MUTE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == OUT_MUTE_A::ZERO
    }
}
#[doc = "Field `out_mute` writer - Data Output Mute Enable"]
pub type OUT_MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, OUT_MUTE_A, O>;
impl<'a, const O: u8> OUT_MUTE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OUT_MUTE_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(OUT_MUTE_A::ZERO)
    }
}
#[doc = "Field `dout_en[0-3]` reader - Data%s Output Enable"]
pub type DOUT_EN_R = crate::BitReader<DOUT_EN_A>;
#[doc = "Data%s Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOUT_EN_A {
    #[doc = "0: Disabled, Hi-Z State"]
    DISABLE = 0,
    #[doc = "1: Enabled"]
    ENABLE = 1,
}
impl From<DOUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DOUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOUT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOUT_EN_A {
        match self.bits {
            false => DOUT_EN_A::DISABLE,
            true => DOUT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DOUT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DOUT_EN_A::ENABLE
    }
}
#[doc = "Field `dout_en[0-3]` writer - Data%s Output Enable"]
pub type DOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, DOUT_EN_A, O>;
impl<'a, const O: u8> DOUT_EN_W<'a, O> {
    #[doc = "Disabled, Hi-Z State"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DOUT_EN_A::DISABLE)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DOUT_EN_A::ENABLE)
    }
}
#[doc = "Field `lrck_out` reader - LRCK Direction Select"]
pub type LRCK_OUT_R = crate::BitReader<LRCK_OUT_A>;
#[doc = "LRCK Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCK_OUT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
}
impl From<LRCK_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: LRCK_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl LRCK_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRCK_OUT_A {
        match self.bits {
            false => LRCK_OUT_A::INPUT,
            true => LRCK_OUT_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == LRCK_OUT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == LRCK_OUT_A::OUTPUT
    }
}
#[doc = "Field `lrck_out` writer - LRCK Direction Select"]
pub type LRCK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, LRCK_OUT_A, O>;
impl<'a, const O: u8> LRCK_OUT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(LRCK_OUT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(LRCK_OUT_A::OUTPUT)
    }
}
#[doc = "Field `bclk_out` reader - Bit Clock Direction Select"]
pub type BCLK_OUT_R = crate::BitReader<BCLK_OUT_A>;
#[doc = "Bit Clock Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCLK_OUT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "1: `1`"]
    OUTPUT = 1,
}
impl From<BCLK_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: BCLK_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl BCLK_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCLK_OUT_A {
        match self.bits {
            false => BCLK_OUT_A::INPUT,
            true => BCLK_OUT_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BCLK_OUT_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == BCLK_OUT_A::OUTPUT
    }
}
#[doc = "Field `bclk_out` writer - Bit Clock Direction Select"]
pub type BCLK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, BCLK_OUT_A, O>;
impl<'a, const O: u8> BCLK_OUT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(BCLK_OUT_A::INPUT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(BCLK_OUT_A::OUTPUT)
    }
}
#[doc = "Field `rx_sync_en` reader - RX Synchronize Enable"]
pub type RX_SYNC_EN_R = crate::BitReader<RX_SYNC_EN_A>;
#[doc = "RX Synchronize Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SYNC_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_SYNC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_SYNC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_SYNC_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_SYNC_EN_A {
        match self.bits {
            false => RX_SYNC_EN_A::DISABLE,
            true => RX_SYNC_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_SYNC_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_SYNC_EN_A::ENABLE
    }
}
#[doc = "Field `rx_sync_en` writer - RX Synchronize Enable"]
pub type RX_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, RX_SYNC_EN_A, O>;
impl<'a, const O: u8> RX_SYNC_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_A::ENABLE)
    }
}
#[doc = "Field `rx_sync_en_start` reader - RX Synchronize Enable Start"]
pub type RX_SYNC_EN_START_R = crate::BitReader<RX_SYNC_EN_START_A>;
#[doc = "RX Synchronize Enable Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SYNC_EN_START_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_SYNC_EN_START_A> for bool {
    #[inline(always)]
    fn from(variant: RX_SYNC_EN_START_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_SYNC_EN_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_SYNC_EN_START_A {
        match self.bits {
            false => RX_SYNC_EN_START_A::DISABLE,
            true => RX_SYNC_EN_START_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_SYNC_EN_START_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_SYNC_EN_START_A::ENABLE
    }
}
#[doc = "Field `rx_sync_en_start` writer - RX Synchronize Enable Start"]
pub type RX_SYNC_EN_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_CTL_SPEC, RX_SYNC_EN_START_A, O>;
impl<'a, const O: u8> RX_SYNC_EN_START_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_START_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_START_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Global Enable"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter Block Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Loopback Test"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Mode Selection"]
    #[inline(always)]
    pub fn mode_sel(&self) -> MODE_SEL_R {
        MODE_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Output Mute Enable"]
    #[inline(always)]
    pub fn out_mute(&self) -> OUT_MUTE_R {
        OUT_MUTE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Data[0-3] Output Enable"]
    #[inline(always)]
    pub unsafe fn dout_en(&self, n: u8) -> DOUT_EN_R {
        DOUT_EN_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - Data0 Output Enable"]
    #[inline(always)]
    pub fn dout0_en(&self) -> DOUT_EN_R {
        DOUT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data1 Output Enable"]
    #[inline(always)]
    pub fn dout1_en(&self) -> DOUT_EN_R {
        DOUT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data2 Output Enable"]
    #[inline(always)]
    pub fn dout2_en(&self) -> DOUT_EN_R {
        DOUT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data3 Output Enable"]
    #[inline(always)]
    pub fn dout3_en(&self) -> DOUT_EN_R {
        DOUT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - LRCK Direction Select"]
    #[inline(always)]
    pub fn lrck_out(&self) -> LRCK_OUT_R {
        LRCK_OUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bit Clock Direction Select"]
    #[inline(always)]
    pub fn bclk_out(&self) -> BCLK_OUT_R {
        BCLK_OUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - RX Synchronize Enable"]
    #[inline(always)]
    pub fn rx_sync_en(&self) -> RX_SYNC_EN_R {
        RX_SYNC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RX Synchronize Enable Start"]
    #[inline(always)]
    pub fn rx_sync_en_start(&self) -> RX_SYNC_EN_START_R {
        RX_SYNC_EN_START_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<0> {
        GEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<1> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter Block Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<2> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 3 - Loopback Test"]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<3> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 4:5 - Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode_sel(&mut self) -> MODE_SEL_W<4> {
        MODE_SEL_W::new(self)
    }
    #[doc = "Bit 6 - Data Output Mute Enable"]
    #[inline(always)]
    #[must_use]
    pub fn out_mute(&mut self) -> OUT_MUTE_W<6> {
        OUT_MUTE_W::new(self)
    }
    #[doc = "Data[0-3] Output Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn dout_en<const O: u8>(&mut self) -> DOUT_EN_W<O> {
        DOUT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Data0 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dout0_en(&mut self) -> DOUT_EN_W<8> {
        DOUT_EN_W::new(self)
    }
    #[doc = "Bit 9 - Data1 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dout1_en(&mut self) -> DOUT_EN_W<9> {
        DOUT_EN_W::new(self)
    }
    #[doc = "Bit 10 - Data2 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dout2_en(&mut self) -> DOUT_EN_W<10> {
        DOUT_EN_W::new(self)
    }
    #[doc = "Bit 11 - Data3 Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dout3_en(&mut self) -> DOUT_EN_W<11> {
        DOUT_EN_W::new(self)
    }
    #[doc = "Bit 17 - LRCK Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn lrck_out(&mut self) -> LRCK_OUT_W<17> {
        LRCK_OUT_W::new(self)
    }
    #[doc = "Bit 18 - Bit Clock Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn bclk_out(&mut self) -> BCLK_OUT_W<18> {
        BCLK_OUT_W::new(self)
    }
    #[doc = "Bit 20 - RX Synchronize Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sync_en(&mut self) -> RX_SYNC_EN_W<20> {
        RX_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 21 - RX Synchronize Enable Start"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sync_en_start(&mut self) -> RX_SYNC_EN_START_W<21> {
        RX_SYNC_EN_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_ctl](index.html) module"]
pub struct I2S_PCM_CTL_SPEC;
impl crate::RegisterSpec for I2S_PCM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_ctl::R](R) reader structure"]
impl crate::Readable for I2S_PCM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_ctl::W](W) writer structure"]
impl crate::Writable for I2S_PCM_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_ctl to value 0"]
impl crate::Resettable for I2S_PCM_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
