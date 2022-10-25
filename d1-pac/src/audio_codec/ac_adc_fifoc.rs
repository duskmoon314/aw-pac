#[doc = "Register `ac_adc_fifoc` reader"]
pub struct R(crate::R<AC_ADC_FIFOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC_ADC_FIFOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC_ADC_FIFOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC_ADC_FIFOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ac_adc_fifoc` writer"]
pub struct W(crate::W<AC_ADC_FIFOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AC_ADC_FIFOC_SPEC>;
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
impl From<crate::W<AC_ADC_FIFOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AC_ADC_FIFOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_fifo_flush` reader - ADC FIFO Flush\n\nWrite '1' to flush TX FIFO, self clear to '0'."]
pub type ADC_FIFO_FLUSH_R = crate::BitReader<bool>;
#[doc = "Field `adc_fifo_flush` writer - ADC FIFO Flush\n\nWrite '1' to flush TX FIFO, self clear to '0'."]
pub type ADC_FIFO_FLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, bool, O>;
#[doc = "Field `adc_overrun_irq_en` reader - ADC FIFO Overrun IRQ Enable"]
pub type ADC_OVERRUN_IRQ_EN_R = crate::BitReader<ADC_OVERRUN_IRQ_EN_A>;
#[doc = "ADC FIFO Overrun IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_OVERRUN_IRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_OVERRUN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => ADC_OVERRUN_IRQ_EN_A::DISABLED,
            true => ADC_OVERRUN_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_OVERRUN_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_OVERRUN_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `adc_overrun_irq_en` writer - ADC FIFO Overrun IRQ Enable"]
pub type ADC_OVERRUN_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, ADC_OVERRUN_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC_OVERRUN_IRQ_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_OVERRUN_IRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_OVERRUN_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_irq_en` reader - ADC FIFO Data Available IRQ Enable"]
pub type ADC_IRQ_EN_R = crate::BitReader<ADC_IRQ_EN_A>;
#[doc = "ADC FIFO Data Available IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_IRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_IRQ_EN_A {
        match self.bits {
            false => ADC_IRQ_EN_A::DISABLED,
            true => ADC_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `adc_irq_en` writer - ADC FIFO Data Available IRQ Enable"]
pub type ADC_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, ADC_IRQ_EN_A, O>;
impl<'a, const O: u8> ADC_IRQ_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_IRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_IRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `adc_drq_en` reader - ADC FIFO Data Available DRQ Enable"]
pub type ADC_DRQ_EN_R = crate::BitReader<ADC_DRQ_EN_A>;
#[doc = "ADC FIFO Data Available DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_DRQ_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADC_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_DRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_DRQ_EN_A {
        match self.bits {
            false => ADC_DRQ_EN_A::DISABLED,
            true => ADC_DRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_DRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_DRQ_EN_A::ENABLED
    }
}
#[doc = "Field `adc_drq_en` writer - ADC FIFO Data Available DRQ Enable"]
pub type ADC_DRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, ADC_DRQ_EN_A, O>;
impl<'a, const O: u8> ADC_DRQ_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_DRQ_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_DRQ_EN_A::ENABLED)
    }
}
#[doc = "Field `rx_fifo_trg_level` reader - RX FIFO Trigger Level (RXTL\\[5:0\\])\n\nInterrupt and DMA request trigger level for RX FIFO normal condition IRQ/DRQ generated when WLEVEL > RXTL\\[5:0\\]"]
pub type RX_FIFO_TRG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_fifo_trg_level` writer - RX FIFO Trigger Level (RXTL\\[5:0\\])\n\nInterrupt and DMA request trigger level for RX FIFO normal condition IRQ/DRQ generated when WLEVEL > RXTL\\[5:0\\]"]
pub type RX_FIFO_TRG_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AC_ADC_FIFOC_SPEC, u8, u8, 8, O>;
#[doc = "Field `rx_sample_bits` reader - Receiving Audio Sample Resolution"]
pub type RX_SAMPLE_BITS_R = crate::BitReader<RX_SAMPLE_BITS_A>;
#[doc = "Receiving Audio Sample Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SAMPLE_BITS_A {
    #[doc = "0: 16 bits"]
    _16_BITS = 0,
    #[doc = "1: 20 bits"]
    _20_BITS = 1,
}
impl From<RX_SAMPLE_BITS_A> for bool {
    #[inline(always)]
    fn from(variant: RX_SAMPLE_BITS_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_SAMPLE_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_SAMPLE_BITS_A {
        match self.bits {
            false => RX_SAMPLE_BITS_A::_16_BITS,
            true => RX_SAMPLE_BITS_A::_20_BITS,
        }
    }
    #[doc = "Checks if the value of the field is `_16_BITS`"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        *self == RX_SAMPLE_BITS_A::_16_BITS
    }
    #[doc = "Checks if the value of the field is `_20_BITS`"]
    #[inline(always)]
    pub fn is_20_bits(&self) -> bool {
        *self == RX_SAMPLE_BITS_A::_20_BITS
    }
}
#[doc = "Field `rx_sample_bits` writer - Receiving Audio Sample Resolution"]
pub type RX_SAMPLE_BITS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, RX_SAMPLE_BITS_A, O>;
impl<'a, const O: u8> RX_SAMPLE_BITS_W<'a, O> {
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut W {
        self.variant(RX_SAMPLE_BITS_A::_16_BITS)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20_bits(self) -> &'a mut W {
        self.variant(RX_SAMPLE_BITS_A::_20_BITS)
    }
}
#[doc = "Field `rx_sync_en` reader - Audiocodec RX Synchronize Enable"]
pub type RX_SYNC_EN_R = crate::BitReader<RX_SYNC_EN_A>;
#[doc = "Audiocodec RX Synchronize Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SYNC_EN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => RX_SYNC_EN_A::DISABLED,
            true => RX_SYNC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_SYNC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_SYNC_EN_A::ENABLED
    }
}
#[doc = "Field `rx_sync_en` writer - Audiocodec RX Synchronize Enable"]
pub type RX_SYNC_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, RX_SYNC_EN_A, O>;
impl<'a, const O: u8> RX_SYNC_EN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_A::ENABLED)
    }
}
#[doc = "Field `rx_sync_en_start` reader - The bit takes effect only when RX_SYNC_EN is set to 1. System Domain: Audio codec/I2S0/I2S1/I2S2/DMIC/OWA RX Synchronize Enable Start."]
pub type RX_SYNC_EN_START_R = crate::BitReader<RX_SYNC_EN_START_A>;
#[doc = "The bit takes effect only when RX_SYNC_EN is set to 1. System Domain: Audio codec/I2S0/I2S1/I2S2/DMIC/OWA RX Synchronize Enable Start.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SYNC_EN_START_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
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
            false => RX_SYNC_EN_START_A::DISABLED,
            true => RX_SYNC_EN_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RX_SYNC_EN_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RX_SYNC_EN_START_A::ENABLED
    }
}
#[doc = "Field `rx_sync_en_start` writer - The bit takes effect only when RX_SYNC_EN is set to 1. System Domain: Audio codec/I2S0/I2S1/I2S2/DMIC/OWA RX Synchronize Enable Start."]
pub type RX_SYNC_EN_START_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, RX_SYNC_EN_START_A, O>;
impl<'a, const O: u8> RX_SYNC_EN_START_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_START_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_SYNC_EN_START_A::ENABLED)
    }
}
#[doc = "Field `rx_fifo_mode` reader - RX FIFO Output Mode (Mode 0, 1) \n\nFor 20-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:0\\], 12'h0}\n\nMode 1: RXDATA\\[31:0\\] = {12{FIFO_O\\[19\\]}, FIFO_O\\[19:0\\]}\n\nFor 16-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:4\\], 16'h0}\n\nMode 1: RXDATA\\[31:0\\] = {16{FIFO_O\\[19\\]}, FIFO_O\\[19:4\\]}"]
pub type RX_FIFO_MODE_R = crate::BitReader<RX_FIFO_MODE_A>;
#[doc = "RX FIFO Output Mode (Mode 0, 1) \n\nFor 20-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:0\\], 12'h0}\n\nMode 1: RXDATA\\[31:0\\] = {12{FIFO_O\\[19\\]}, FIFO_O\\[19:0\\]}\n\nFor 16-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:4\\], 16'h0}\n\nMode 1: RXDATA\\[31:0\\] = {16{FIFO_O\\[19\\]}, FIFO_O\\[19:4\\]}\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_MODE_A {
    #[doc = "0: Expanding '0' at LSB of TX FIFO register"]
    E_XPANDING_ZERO = 0,
    #[doc = "1: Expanding received sample sign bit at MSB of TX FIFO register"]
    E_XPANDING_SIGN = 1,
}
impl From<RX_FIFO_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_MODE_A {
        match self.bits {
            false => RX_FIFO_MODE_A::E_XPANDING_ZERO,
            true => RX_FIFO_MODE_A::E_XPANDING_SIGN,
        }
    }
    #[doc = "Checks if the value of the field is `E_XPANDING_ZERO`"]
    #[inline(always)]
    pub fn is_e_xpanding_zero(&self) -> bool {
        *self == RX_FIFO_MODE_A::E_XPANDING_ZERO
    }
    #[doc = "Checks if the value of the field is `E_XPANDING_SIGN`"]
    #[inline(always)]
    pub fn is_e_xpanding_sign(&self) -> bool {
        *self == RX_FIFO_MODE_A::E_XPANDING_SIGN
    }
}
#[doc = "Field `rx_fifo_mode` writer - RX FIFO Output Mode (Mode 0, 1) \n\nFor 20-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:0\\], 12'h0}\n\nMode 1: RXDATA\\[31:0\\] = {12{FIFO_O\\[19\\]}, FIFO_O\\[19:0\\]}\n\nFor 16-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:4\\], 16'h0}\n\nMode 1: RXDATA\\[31:0\\] = {16{FIFO_O\\[19\\]}, FIFO_O\\[19:4\\]}"]
pub type RX_FIFO_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, RX_FIFO_MODE_A, O>;
impl<'a, const O: u8> RX_FIFO_MODE_W<'a, O> {
    #[doc = "Expanding '0' at LSB of TX FIFO register"]
    #[inline(always)]
    pub fn e_xpanding_zero(self) -> &'a mut W {
        self.variant(RX_FIFO_MODE_A::E_XPANDING_ZERO)
    }
    #[doc = "Expanding received sample sign bit at MSB of TX FIFO register"]
    #[inline(always)]
    pub fn e_xpanding_sign(self) -> &'a mut W {
        self.variant(RX_FIFO_MODE_A::E_XPANDING_SIGN)
    }
}
#[doc = "Field `adcdfen` reader - ADC FIFO delay function for writing data after EN_AD"]
pub type ADCDFEN_R = crate::BitReader<ADCDFEN_A>;
#[doc = "ADC FIFO delay function for writing data after EN_AD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCDFEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADCDFEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCDFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCDFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCDFEN_A {
        match self.bits {
            false => ADCDFEN_A::DISABLED,
            true => ADCDFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCDFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCDFEN_A::ENABLED
    }
}
#[doc = "Field `adcdfen` writer - ADC FIFO delay function for writing data after EN_AD"]
pub type ADCDFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, ADCDFEN_A, O>;
impl<'a, const O: u8> ADCDFEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCDFEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCDFEN_A::ENABLED)
    }
}
#[doc = "Field `adcfdt` reader - ADC FIFO delay time for writing data after EN_AD"]
pub type ADCFDT_R = crate::FieldReader<u8, ADCFDT_A>;
#[doc = "ADC FIFO delay time for writing data after EN_AD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCFDT_A {
    #[doc = "0: 5 ms"]
    _5_MS = 0,
    #[doc = "1: 10 ms"]
    _10_MS = 1,
    #[doc = "2: 20 ms"]
    _20_MS = 2,
    #[doc = "3: 30 ms"]
    _30_MS = 3,
}
impl From<ADCFDT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCFDT_A) -> Self {
        variant as _
    }
}
impl ADCFDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCFDT_A {
        match self.bits {
            0 => ADCFDT_A::_5_MS,
            1 => ADCFDT_A::_10_MS,
            2 => ADCFDT_A::_20_MS,
            3 => ADCFDT_A::_30_MS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_MS`"]
    #[inline(always)]
    pub fn is_5_ms(&self) -> bool {
        *self == ADCFDT_A::_5_MS
    }
    #[doc = "Checks if the value of the field is `_10_MS`"]
    #[inline(always)]
    pub fn is_10_ms(&self) -> bool {
        *self == ADCFDT_A::_10_MS
    }
    #[doc = "Checks if the value of the field is `_20_MS`"]
    #[inline(always)]
    pub fn is_20_ms(&self) -> bool {
        *self == ADCFDT_A::_20_MS
    }
    #[doc = "Checks if the value of the field is `_30_MS`"]
    #[inline(always)]
    pub fn is_30_ms(&self) -> bool {
        *self == ADCFDT_A::_30_MS
    }
}
#[doc = "Field `adcfdt` writer - ADC FIFO delay time for writing data after EN_AD"]
pub type ADCFDT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AC_ADC_FIFOC_SPEC, u8, ADCFDT_A, 2, O>;
impl<'a, const O: u8> ADCFDT_W<'a, O> {
    #[doc = "5 ms"]
    #[inline(always)]
    pub fn _5_ms(self) -> &'a mut W {
        self.variant(ADCFDT_A::_5_MS)
    }
    #[doc = "10 ms"]
    #[inline(always)]
    pub fn _10_ms(self) -> &'a mut W {
        self.variant(ADCFDT_A::_10_MS)
    }
    #[doc = "20 ms"]
    #[inline(always)]
    pub fn _20_ms(self) -> &'a mut W {
        self.variant(ADCFDT_A::_20_MS)
    }
    #[doc = "30 ms"]
    #[inline(always)]
    pub fn _30_ms(self) -> &'a mut W {
        self.variant(ADCFDT_A::_30_MS)
    }
}
#[doc = "Field `en_ad` reader - ADC Digital Part Enable"]
pub type EN_AD_R = crate::BitReader<EN_AD_A>;
#[doc = "ADC Digital Part Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_AD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<EN_AD_A> for bool {
    #[inline(always)]
    fn from(variant: EN_AD_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_AD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_AD_A {
        match self.bits {
            false => EN_AD_A::DISABLED,
            true => EN_AD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_AD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_AD_A::ENABLED
    }
}
#[doc = "Field `en_ad` writer - ADC Digital Part Enable"]
pub type EN_AD_W<'a, const O: u8> = crate::BitWriter<'a, u32, AC_ADC_FIFOC_SPEC, EN_AD_A, O>;
impl<'a, const O: u8> EN_AD_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_AD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_AD_A::ENABLED)
    }
}
#[doc = "Field `adfs` reader - Sample Rate of ADC \n\n44.1 kHz/22.05 kHz/11.025 kHz can be supported by Audio PLL Configure Bit."]
pub type ADFS_R = crate::FieldReader<u8, ADFS_A>;
#[doc = "Sample Rate of ADC \n\n44.1 kHz/22.05 kHz/11.025 kHz can be supported by Audio PLL Configure Bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADFS_A {
    #[doc = "0: 48 kHz"]
    _48KHZ = 0,
    #[doc = "2: 24 kHz"]
    _24KHZ = 2,
    #[doc = "4: 12 kHz"]
    _12KHZ = 4,
    #[doc = "1: 32 kHz"]
    _32KHZ = 1,
    #[doc = "3: 16 kHz"]
    _16KHZ = 3,
    #[doc = "5: 8 kHz"]
    _8KHZ = 5,
    #[doc = "7: Reserved"]
    R_ESERVED = 7,
}
impl From<ADFS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADFS_A) -> Self {
        variant as _
    }
}
impl ADFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADFS_A> {
        match self.bits {
            0 => Some(ADFS_A::_48KHZ),
            2 => Some(ADFS_A::_24KHZ),
            4 => Some(ADFS_A::_12KHZ),
            1 => Some(ADFS_A::_32KHZ),
            3 => Some(ADFS_A::_16KHZ),
            5 => Some(ADFS_A::_8KHZ),
            7 => Some(ADFS_A::R_ESERVED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_48KHZ`"]
    #[inline(always)]
    pub fn is_48khz(&self) -> bool {
        *self == ADFS_A::_48KHZ
    }
    #[doc = "Checks if the value of the field is `_24KHZ`"]
    #[inline(always)]
    pub fn is_24khz(&self) -> bool {
        *self == ADFS_A::_24KHZ
    }
    #[doc = "Checks if the value of the field is `_12KHZ`"]
    #[inline(always)]
    pub fn is_12khz(&self) -> bool {
        *self == ADFS_A::_12KHZ
    }
    #[doc = "Checks if the value of the field is `_32KHZ`"]
    #[inline(always)]
    pub fn is_32khz(&self) -> bool {
        *self == ADFS_A::_32KHZ
    }
    #[doc = "Checks if the value of the field is `_16KHZ`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == ADFS_A::_16KHZ
    }
    #[doc = "Checks if the value of the field is `_8KHZ`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == ADFS_A::_8KHZ
    }
    #[doc = "Checks if the value of the field is `R_ESERVED`"]
    #[inline(always)]
    pub fn is_r_eserved(&self) -> bool {
        *self == ADFS_A::R_ESERVED
    }
}
#[doc = "Field `adfs` writer - Sample Rate of ADC \n\n44.1 kHz/22.05 kHz/11.025 kHz can be supported by Audio PLL Configure Bit."]
pub type ADFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AC_ADC_FIFOC_SPEC, u8, ADFS_A, 3, O>;
impl<'a, const O: u8> ADFS_W<'a, O> {
    #[doc = "48 kHz"]
    #[inline(always)]
    pub fn _48khz(self) -> &'a mut W {
        self.variant(ADFS_A::_48KHZ)
    }
    #[doc = "24 kHz"]
    #[inline(always)]
    pub fn _24khz(self) -> &'a mut W {
        self.variant(ADFS_A::_24KHZ)
    }
    #[doc = "12 kHz"]
    #[inline(always)]
    pub fn _12khz(self) -> &'a mut W {
        self.variant(ADFS_A::_12KHZ)
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn _32khz(self) -> &'a mut W {
        self.variant(ADFS_A::_32KHZ)
    }
    #[doc = "16 kHz"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut W {
        self.variant(ADFS_A::_16KHZ)
    }
    #[doc = "8 kHz"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut W {
        self.variant(ADFS_A::_8KHZ)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn r_eserved(self) -> &'a mut W {
        self.variant(ADFS_A::R_ESERVED)
    }
}
impl R {
    #[doc = "Bit 0 - ADC FIFO Flush\n\nWrite '1' to flush TX FIFO, self clear to '0'."]
    #[inline(always)]
    pub fn adc_fifo_flush(&self) -> ADC_FIFO_FLUSH_R {
        ADC_FIFO_FLUSH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    pub fn adc_overrun_irq_en(&self) -> ADC_OVERRUN_IRQ_EN_R {
        ADC_OVERRUN_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    pub fn adc_irq_en(&self) -> ADC_IRQ_EN_R {
        ADC_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC FIFO Data Available DRQ Enable"]
    #[inline(always)]
    pub fn adc_drq_en(&self) -> ADC_DRQ_EN_R {
        ADC_DRQ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - RX FIFO Trigger Level (RXTL\\[5:0\\])\n\nInterrupt and DMA request trigger level for RX FIFO normal condition IRQ/DRQ generated when WLEVEL > RXTL\\[5:0\\]"]
    #[inline(always)]
    pub fn rx_fifo_trg_level(&self) -> RX_FIFO_TRG_LEVEL_R {
        RX_FIFO_TRG_LEVEL_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Receiving Audio Sample Resolution"]
    #[inline(always)]
    pub fn rx_sample_bits(&self) -> RX_SAMPLE_BITS_R {
        RX_SAMPLE_BITS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Audiocodec RX Synchronize Enable"]
    #[inline(always)]
    pub fn rx_sync_en(&self) -> RX_SYNC_EN_R {
        RX_SYNC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit takes effect only when RX_SYNC_EN is set to 1. System Domain: Audio codec/I2S0/I2S1/I2S2/DMIC/OWA RX Synchronize Enable Start."]
    #[inline(always)]
    pub fn rx_sync_en_start(&self) -> RX_SYNC_EN_START_R {
        RX_SYNC_EN_START_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - RX FIFO Output Mode (Mode 0, 1) \n\nFor 20-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:0\\], 12'h0}\n\nMode 1: RXDATA\\[31:0\\] = {12{FIFO_O\\[19\\]}, FIFO_O\\[19:0\\]}\n\nFor 16-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:4\\], 16'h0}\n\nMode 1: RXDATA\\[31:0\\] = {16{FIFO_O\\[19\\]}, FIFO_O\\[19:4\\]}"]
    #[inline(always)]
    pub fn rx_fifo_mode(&self) -> RX_FIFO_MODE_R {
        RX_FIFO_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC FIFO delay function for writing data after EN_AD"]
    #[inline(always)]
    pub fn adcdfen(&self) -> ADCDFEN_R {
        ADCDFEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - ADC FIFO delay time for writing data after EN_AD"]
    #[inline(always)]
    pub fn adcfdt(&self) -> ADCFDT_R {
        ADCFDT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - ADC Digital Part Enable"]
    #[inline(always)]
    pub fn en_ad(&self) -> EN_AD_R {
        EN_AD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Sample Rate of ADC \n\n44.1 kHz/22.05 kHz/11.025 kHz can be supported by Audio PLL Configure Bit."]
    #[inline(always)]
    pub fn adfs(&self) -> ADFS_R {
        ADFS_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC FIFO Flush\n\nWrite '1' to flush TX FIFO, self clear to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_flush(&mut self) -> ADC_FIFO_FLUSH_W<0> {
        ADC_FIFO_FLUSH_W::new(self)
    }
    #[doc = "Bit 1 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_overrun_irq_en(&mut self) -> ADC_OVERRUN_IRQ_EN_W<1> {
        ADC_OVERRUN_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq_en(&mut self) -> ADC_IRQ_EN_W<2> {
        ADC_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 3 - ADC FIFO Data Available DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drq_en(&mut self) -> ADC_DRQ_EN_W<3> {
        ADC_DRQ_EN_W::new(self)
    }
    #[doc = "Bits 4:11 - RX FIFO Trigger Level (RXTL\\[5:0\\])\n\nInterrupt and DMA request trigger level for RX FIFO normal condition IRQ/DRQ generated when WLEVEL > RXTL\\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trg_level(&mut self) -> RX_FIFO_TRG_LEVEL_W<4> {
        RX_FIFO_TRG_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - Receiving Audio Sample Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sample_bits(&mut self) -> RX_SAMPLE_BITS_W<16> {
        RX_SAMPLE_BITS_W::new(self)
    }
    #[doc = "Bit 20 - Audiocodec RX Synchronize Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sync_en(&mut self) -> RX_SYNC_EN_W<20> {
        RX_SYNC_EN_W::new(self)
    }
    #[doc = "Bit 21 - The bit takes effect only when RX_SYNC_EN is set to 1. System Domain: Audio codec/I2S0/I2S1/I2S2/DMIC/OWA RX Synchronize Enable Start."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sync_en_start(&mut self) -> RX_SYNC_EN_START_W<21> {
        RX_SYNC_EN_START_W::new(self)
    }
    #[doc = "Bit 24 - RX FIFO Output Mode (Mode 0, 1) \n\nFor 20-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:0\\], 12'h0}\n\nMode 1: RXDATA\\[31:0\\] = {12{FIFO_O\\[19\\]}, FIFO_O\\[19:0\\]}\n\nFor 16-bit received audio sample:\n\nMode 0: RXDATA\\[31:0\\] = {FIFO_O\\[19:4\\], 16'h0}\n\nMode 1: RXDATA\\[31:0\\] = {16{FIFO_O\\[19\\]}, FIFO_O\\[19:4\\]}"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mode(&mut self) -> RX_FIFO_MODE_W<24> {
        RX_FIFO_MODE_W::new(self)
    }
    #[doc = "Bit 25 - ADC FIFO delay function for writing data after EN_AD"]
    #[inline(always)]
    #[must_use]
    pub fn adcdfen(&mut self) -> ADCDFEN_W<25> {
        ADCDFEN_W::new(self)
    }
    #[doc = "Bits 26:27 - ADC FIFO delay time for writing data after EN_AD"]
    #[inline(always)]
    #[must_use]
    pub fn adcfdt(&mut self) -> ADCFDT_W<26> {
        ADCFDT_W::new(self)
    }
    #[doc = "Bit 28 - ADC Digital Part Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_ad(&mut self) -> EN_AD_W<28> {
        EN_AD_W::new(self)
    }
    #[doc = "Bits 29:31 - Sample Rate of ADC \n\n44.1 kHz/22.05 kHz/11.025 kHz can be supported by Audio PLL Configure Bit."]
    #[inline(always)]
    #[must_use]
    pub fn adfs(&mut self) -> ADFS_W<29> {
        ADFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac_adc_fifoc](index.html) module"]
pub struct AC_ADC_FIFOC_SPEC;
impl crate::RegisterSpec for AC_ADC_FIFOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac_adc_fifoc::R](R) reader structure"]
impl crate::Readable for AC_ADC_FIFOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ac_adc_fifoc::W](W) writer structure"]
impl crate::Writable for AC_ADC_FIFOC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_fifoc to value 0x0400"]
impl crate::Resettable for AC_ADC_FIFOC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
