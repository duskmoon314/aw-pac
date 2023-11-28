#[doc = "Register `ac_dac_fifoc` reader"]
pub type R = crate::R<AC_DAC_FIFOC_SPEC>;
#[doc = "Register `ac_dac_fifoc` writer"]
pub type W = crate::W<AC_DAC_FIFOC_SPEC>;
#[doc = "Field `fifo_flush` reader - DAC FIFO Flush"]
pub type FIFO_FLUSH_R = crate::BitReader<FIFO_FLUSH_A>;
#[doc = "DAC FIFO Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_FLUSH_A {
    #[doc = "0: `0`"]
    SELF_CLEAR = 0,
    #[doc = "1: `1`"]
    FLUSH = 1,
}
impl From<FIFO_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_FLUSH_A {
        match self.bits {
            false => FIFO_FLUSH_A::SELF_CLEAR,
            true => FIFO_FLUSH_A::FLUSH,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_self_clear(&self) -> bool {
        *self == FIFO_FLUSH_A::SELF_CLEAR
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FIFO_FLUSH_A::FLUSH
    }
}
#[doc = "Field `fifo_flush` writer - DAC FIFO Flush"]
pub type FIFO_FLUSH_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_FLUSH_A>;
impl<'a, REG> FIFO_FLUSH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn self_clear(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_FLUSH_A::SELF_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_FLUSH_A::FLUSH)
    }
}
#[doc = "Field `fifo_overrun_irq_en` reader - DAC FIFO Overrun IRQ Enable"]
pub type FIFO_OVERRUN_IRQ_EN_R = crate::BitReader<FIFO_OVERRUN_IRQ_EN_A>;
#[doc = "DAC FIFO Overrun IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_OVERRUN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_OVERRUN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => FIFO_OVERRUN_IRQ_EN_A::DISABLE,
            true => FIFO_OVERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_OVERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_OVERRUN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_overrun_irq_en` writer - DAC FIFO Overrun IRQ Enable"]
pub type FIFO_OVERRUN_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_OVERRUN_IRQ_EN_A>;
impl<'a, REG> FIFO_OVERRUN_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `fifo_underrun_irq_en` reader - DAC FIFO Underrun IRQ Enable"]
pub type FIFO_UNDERRUN_IRQ_EN_R = crate::BitReader<FIFO_UNDERRUN_IRQ_EN_A>;
#[doc = "DAC FIFO Underrun IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFO_UNDERRUN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FIFO_UNDERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_UNDERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFO_UNDERRUN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_UNDERRUN_IRQ_EN_A {
        match self.bits {
            false => FIFO_UNDERRUN_IRQ_EN_A::DISABLE,
            true => FIFO_UNDERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIFO_UNDERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIFO_UNDERRUN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `fifo_underrun_irq_en` writer - DAC FIFO Underrun IRQ Enable"]
pub type FIFO_UNDERRUN_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, FIFO_UNDERRUN_IRQ_EN_A>;
impl<'a, REG> FIFO_UNDERRUN_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_UNDERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_UNDERRUN_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `dac_irq_en` reader - DAC FIFO Empty IRQ Enable"]
pub type DAC_IRQ_EN_R = crate::BitReader<DAC_IRQ_EN_A>;
#[doc = "DAC FIFO Empty IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DAC_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_IRQ_EN_A {
        match self.bits {
            false => DAC_IRQ_EN_A::DISABLE,
            true => DAC_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DAC_IRQ_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DAC_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `dac_irq_en` writer - DAC FIFO Empty IRQ Enable"]
pub type DAC_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, DAC_IRQ_EN_A>;
impl<'a, REG> DAC_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `dac_drq_en` reader - DAC FIFO Empty DRQ Enable"]
pub type DAC_DRQ_EN_R = crate::BitReader<DAC_DRQ_EN_A>;
#[doc = "DAC FIFO Empty DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_DRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DAC_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_DRQ_EN_A {
        match self.bits {
            false => DAC_DRQ_EN_A::DISABLE,
            true => DAC_DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DAC_DRQ_EN_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DAC_DRQ_EN_A::ENABLE
    }
}
#[doc = "Field `dac_drq_en` writer - DAC FIFO Empty DRQ Enable"]
pub type DAC_DRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, DAC_DRQ_EN_A>;
impl<'a, REG> DAC_DRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `tx_sample_bits` reader - Transmitting Audio Sample Resolution"]
pub type TX_SAMPLE_BITS_R = crate::BitReader<TX_SAMPLE_BITS_A>;
#[doc = "Transmitting Audio Sample Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_SAMPLE_BITS_A {
    #[doc = "0: `0`"]
    BITS_16 = 0,
    #[doc = "1: `1`"]
    BITS_20 = 1,
}
impl From<TX_SAMPLE_BITS_A> for bool {
    #[inline(always)]
    fn from(variant: TX_SAMPLE_BITS_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_SAMPLE_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_SAMPLE_BITS_A {
        match self.bits {
            false => TX_SAMPLE_BITS_A::BITS_16,
            true => TX_SAMPLE_BITS_A::BITS_20,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == TX_SAMPLE_BITS_A::BITS_16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_bits_20(&self) -> bool {
        *self == TX_SAMPLE_BITS_A::BITS_20
    }
}
#[doc = "Field `tx_sample_bits` writer - Transmitting Audio Sample Resolution"]
pub type TX_SAMPLE_BITS_W<'a, REG> = crate::BitWriter<'a, REG, TX_SAMPLE_BITS_A>;
impl<'a, REG> TX_SAMPLE_BITS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SAMPLE_BITS_A::BITS_16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn bits_20(self) -> &'a mut crate::W<REG> {
        self.variant(TX_SAMPLE_BITS_A::BITS_20)
    }
}
#[doc = "Field `dac_mono_en` reader - DAC Mono Enable"]
pub type DAC_MONO_EN_R = crate::BitReader<DAC_MONO_EN_A>;
#[doc = "DAC Mono Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC_MONO_EN_A {
    #[doc = "0: `0`"]
    STEREO = 0,
    #[doc = "1: `1`"]
    MONO = 1,
}
impl From<DAC_MONO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DAC_MONO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC_MONO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_MONO_EN_A {
        match self.bits {
            false => DAC_MONO_EN_A::STEREO,
            true => DAC_MONO_EN_A::MONO,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == DAC_MONO_EN_A::STEREO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == DAC_MONO_EN_A::MONO
    }
}
#[doc = "Field `dac_mono_en` writer - DAC Mono Enable"]
pub type DAC_MONO_EN_W<'a, REG> = crate::BitWriter<'a, REG, DAC_MONO_EN_A>;
impl<'a, REG> DAC_MONO_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_MONO_EN_A::STEREO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_MONO_EN_A::MONO)
    }
}
#[doc = "Field `dac_drq_clr_cnt` reader - DAC DRQ clear count"]
pub type DAC_DRQ_CLR_CNT_R = crate::FieldReader<DAC_DRQ_CLR_CNT_A>;
#[doc = "DAC DRQ clear count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_DRQ_CLR_CNT_A {
    #[doc = "0: `0`"]
    WLEVEL = 0,
    #[doc = "1: `1`"]
    N4 = 1,
    #[doc = "2: `10`"]
    N8 = 2,
    #[doc = "3: `11`"]
    N16 = 3,
}
impl From<DAC_DRQ_CLR_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_DRQ_CLR_CNT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_DRQ_CLR_CNT_A {
    type Ux = u8;
}
impl DAC_DRQ_CLR_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DAC_DRQ_CLR_CNT_A> {
        match self.bits {
            0 => Some(DAC_DRQ_CLR_CNT_A::WLEVEL),
            1 => Some(DAC_DRQ_CLR_CNT_A::N4),
            2 => Some(DAC_DRQ_CLR_CNT_A::N8),
            3 => Some(DAC_DRQ_CLR_CNT_A::N16),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_wlevel(&self) -> bool {
        *self == DAC_DRQ_CLR_CNT_A::WLEVEL
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_n4(&self) -> bool {
        *self == DAC_DRQ_CLR_CNT_A::N4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_n8(&self) -> bool {
        *self == DAC_DRQ_CLR_CNT_A::N8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_n16(&self) -> bool {
        *self == DAC_DRQ_CLR_CNT_A::N16
    }
}
#[doc = "Field `dac_drq_clr_cnt` writer - DAC DRQ clear count"]
pub type DAC_DRQ_CLR_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7, DAC_DRQ_CLR_CNT_A>;
impl<'a, REG> DAC_DRQ_CLR_CNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn wlevel(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_CLR_CNT_A::WLEVEL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn n4(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_CLR_CNT_A::N4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn n8(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_CLR_CNT_A::N8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn n16(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_DRQ_CLR_CNT_A::N16)
    }
}
#[doc = "Field `tx_trig_level` reader - TX FIFO Empty Trigger Level"]
pub type TX_TRIG_LEVEL_R = crate::FieldReader;
#[doc = "Field `tx_trig_level` writer - TX FIFO Empty Trigger Level"]
pub type TX_TRIG_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `fifo_mode` reader - "]
pub type FIFO_MODE_R = crate::FieldReader<FIFO_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFO_MODE_A {
    #[doc = "0: `0`"]
    BIG_ENDIAN = 0,
    #[doc = "1: `1`"]
    LITTLE_ENDIAN = 1,
}
impl From<FIFO_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFO_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FIFO_MODE_A {
    type Ux = u8;
}
impl FIFO_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FIFO_MODE_A> {
        match self.bits {
            0 => Some(FIFO_MODE_A::BIG_ENDIAN),
            1 => Some(FIFO_MODE_A::LITTLE_ENDIAN),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_big_endian(&self) -> bool {
        *self == FIFO_MODE_A::BIG_ENDIAN
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_little_endian(&self) -> bool {
        *self == FIFO_MODE_A::LITTLE_ENDIAN
    }
}
#[doc = "Field `fifo_mode` writer - "]
pub type FIFO_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FIFO_MODE_A>;
impl<'a, REG> FIFO_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn big_endian(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_MODE_A::BIG_ENDIAN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn little_endian(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_MODE_A::LITTLE_ENDIAN)
    }
}
#[doc = "Field `send_last` reader - Audio sample select when TX FIFO underrun"]
pub type SEND_LAST_R = crate::BitReader<SEND_LAST_A>;
#[doc = "Audio sample select when TX FIFO underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEND_LAST_A {
    #[doc = "0: `0`"]
    ZERO = 0,
    #[doc = "1: `1`"]
    LAST = 1,
}
impl From<SEND_LAST_A> for bool {
    #[inline(always)]
    fn from(variant: SEND_LAST_A) -> Self {
        variant as u8 != 0
    }
}
impl SEND_LAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEND_LAST_A {
        match self.bits {
            false => SEND_LAST_A::ZERO,
            true => SEND_LAST_A::LAST,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SEND_LAST_A::ZERO
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == SEND_LAST_A::LAST
    }
}
#[doc = "Field `send_last` writer - Audio sample select when TX FIFO underrun"]
pub type SEND_LAST_W<'a, REG> = crate::BitWriter<'a, REG, SEND_LAST_A>;
impl<'a, REG> SEND_LAST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(SEND_LAST_A::ZERO)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(SEND_LAST_A::LAST)
    }
}
#[doc = "Field `fir_ver` reader - FIR Version"]
pub type FIR_VER_R = crate::BitReader<FIR_VER_A>;
#[doc = "FIR Version\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIR_VER_A {
    #[doc = "0: `0`"]
    TAP_64 = 0,
    #[doc = "1: `1`"]
    TAP_32 = 1,
}
impl From<FIR_VER_A> for bool {
    #[inline(always)]
    fn from(variant: FIR_VER_A) -> Self {
        variant as u8 != 0
    }
}
impl FIR_VER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIR_VER_A {
        match self.bits {
            false => FIR_VER_A::TAP_64,
            true => FIR_VER_A::TAP_32,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_tap_64(&self) -> bool {
        *self == FIR_VER_A::TAP_64
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_tap_32(&self) -> bool {
        *self == FIR_VER_A::TAP_32
    }
}
#[doc = "Field `fir_ver` writer - FIR Version"]
pub type FIR_VER_W<'a, REG> = crate::BitWriter<'a, REG, FIR_VER_A>;
impl<'a, REG> FIR_VER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn tap_64(self) -> &'a mut crate::W<REG> {
        self.variant(FIR_VER_A::TAP_64)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn tap_32(self) -> &'a mut crate::W<REG> {
        self.variant(FIR_VER_A::TAP_32)
    }
}
#[doc = "Field `dac_fs` reader - Sample Rate of DAC"]
pub type DAC_FS_R = crate::FieldReader<DAC_FS_A>;
#[doc = "Sample Rate of DAC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAC_FS_A {
    #[doc = "0: `0`"]
    FS48K = 0,
    #[doc = "1: `1`"]
    FS32K = 1,
    #[doc = "2: `10`"]
    FS24K = 2,
    #[doc = "3: `11`"]
    FS16K = 3,
    #[doc = "4: `100`"]
    FS12K = 4,
    #[doc = "5: `101`"]
    FS8K = 5,
    #[doc = "6: `110`"]
    FS192K = 6,
    #[doc = "7: `111`"]
    FS96K = 7,
}
impl From<DAC_FS_A> for u8 {
    #[inline(always)]
    fn from(variant: DAC_FS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAC_FS_A {
    type Ux = u8;
}
impl DAC_FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAC_FS_A {
        match self.bits {
            0 => DAC_FS_A::FS48K,
            1 => DAC_FS_A::FS32K,
            2 => DAC_FS_A::FS24K,
            3 => DAC_FS_A::FS16K,
            4 => DAC_FS_A::FS12K,
            5 => DAC_FS_A::FS8K,
            6 => DAC_FS_A::FS192K,
            7 => DAC_FS_A::FS96K,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_fs48k(&self) -> bool {
        *self == DAC_FS_A::FS48K
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_fs32k(&self) -> bool {
        *self == DAC_FS_A::FS32K
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_fs24k(&self) -> bool {
        *self == DAC_FS_A::FS24K
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_fs16k(&self) -> bool {
        *self == DAC_FS_A::FS16K
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_fs12k(&self) -> bool {
        *self == DAC_FS_A::FS12K
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_fs8k(&self) -> bool {
        *self == DAC_FS_A::FS8K
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_fs192k(&self) -> bool {
        *self == DAC_FS_A::FS192K
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_fs96k(&self) -> bool {
        *self == DAC_FS_A::FS96K
    }
}
#[doc = "Field `dac_fs` writer - Sample Rate of DAC"]
pub type DAC_FS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DAC_FS_A>;
impl<'a, REG> DAC_FS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fs48k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS48K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fs32k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS32K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fs24k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS24K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fs16k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS16K)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn fs12k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS12K)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn fs8k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS8K)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn fs192k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS192K)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn fs96k(self) -> &'a mut crate::W<REG> {
        self.variant(DAC_FS_A::FS96K)
    }
}
impl R {
    #[doc = "Bit 0 - DAC FIFO Flush"]
    #[inline(always)]
    pub fn fifo_flush(&self) -> FIFO_FLUSH_R {
        FIFO_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    pub fn fifo_overrun_irq_en(&self) -> FIFO_OVERRUN_IRQ_EN_R {
        FIFO_OVERRUN_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC FIFO Underrun IRQ Enable"]
    #[inline(always)]
    pub fn fifo_underrun_irq_en(&self) -> FIFO_UNDERRUN_IRQ_EN_R {
        FIFO_UNDERRUN_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC FIFO Empty IRQ Enable"]
    #[inline(always)]
    pub fn dac_irq_en(&self) -> DAC_IRQ_EN_R {
        DAC_IRQ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC FIFO Empty DRQ Enable"]
    #[inline(always)]
    pub fn dac_drq_en(&self) -> DAC_DRQ_EN_R {
        DAC_DRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitting Audio Sample Resolution"]
    #[inline(always)]
    pub fn tx_sample_bits(&self) -> TX_SAMPLE_BITS_R {
        TX_SAMPLE_BITS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC Mono Enable"]
    #[inline(always)]
    pub fn dac_mono_en(&self) -> DAC_MONO_EN_R {
        DAC_MONO_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:14 - DAC DRQ clear count"]
    #[inline(always)]
    pub fn dac_drq_clr_cnt(&self) -> DAC_DRQ_CLR_CNT_R {
        DAC_DRQ_CLR_CNT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - TX FIFO Empty Trigger Level"]
    #[inline(always)]
    pub fn tx_trig_level(&self) -> TX_TRIG_LEVEL_R {
        TX_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Audio sample select when TX FIFO underrun"]
    #[inline(always)]
    pub fn send_last(&self) -> SEND_LAST_R {
        SEND_LAST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - FIR Version"]
    #[inline(always)]
    pub fn fir_ver(&self) -> FIR_VER_R {
        FIR_VER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Sample Rate of DAC"]
    #[inline(always)]
    pub fn dac_fs(&self) -> DAC_FS_R {
        DAC_FS_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DAC FIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_flush(&mut self) -> FIFO_FLUSH_W<AC_DAC_FIFOC_SPEC> {
        FIFO_FLUSH_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_irq_en(&mut self) -> FIFO_OVERRUN_IRQ_EN_W<AC_DAC_FIFOC_SPEC> {
        FIFO_OVERRUN_IRQ_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC FIFO Underrun IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underrun_irq_en(&mut self) -> FIFO_UNDERRUN_IRQ_EN_W<AC_DAC_FIFOC_SPEC> {
        FIFO_UNDERRUN_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DAC FIFO Empty IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_irq_en(&mut self) -> DAC_IRQ_EN_W<AC_DAC_FIFOC_SPEC> {
        DAC_IRQ_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - DAC FIFO Empty DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drq_en(&mut self) -> DAC_DRQ_EN_W<AC_DAC_FIFOC_SPEC> {
        DAC_DRQ_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Transmitting Audio Sample Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sample_bits(&mut self) -> TX_SAMPLE_BITS_W<AC_DAC_FIFOC_SPEC> {
        TX_SAMPLE_BITS_W::new(self, 5)
    }
    #[doc = "Bit 6 - DAC Mono Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_mono_en(&mut self) -> DAC_MONO_EN_W<AC_DAC_FIFOC_SPEC> {
        DAC_MONO_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:14 - DAC DRQ clear count"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drq_clr_cnt(&mut self) -> DAC_DRQ_CLR_CNT_W<AC_DAC_FIFOC_SPEC> {
        DAC_DRQ_CLR_CNT_W::new(self, 8)
    }
    #[doc = "Bits 8:14 - TX FIFO Empty Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trig_level(&mut self) -> TX_TRIG_LEVEL_W<AC_DAC_FIFOC_SPEC> {
        TX_TRIG_LEVEL_W::new(self, 8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W<AC_DAC_FIFOC_SPEC> {
        FIFO_MODE_W::new(self, 24)
    }
    #[doc = "Bit 26 - Audio sample select when TX FIFO underrun"]
    #[inline(always)]
    #[must_use]
    pub fn send_last(&mut self) -> SEND_LAST_W<AC_DAC_FIFOC_SPEC> {
        SEND_LAST_W::new(self, 26)
    }
    #[doc = "Bit 28 - FIR Version"]
    #[inline(always)]
    #[must_use]
    pub fn fir_ver(&mut self) -> FIR_VER_W<AC_DAC_FIFOC_SPEC> {
        FIR_VER_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - Sample Rate of DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dac_fs(&mut self) -> DAC_FS_W<AC_DAC_FIFOC_SPEC> {
        DAC_FS_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_fifoc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_fifoc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_FIFOC_SPEC;
impl crate::RegisterSpec for AC_DAC_FIFOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_fifoc::R`](R) reader structure"]
impl crate::Readable for AC_DAC_FIFOC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_fifoc::W`](W) writer structure"]
impl crate::Writable for AC_DAC_FIFOC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_fifoc to value 0"]
impl crate::Resettable for AC_DAC_FIFOC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
