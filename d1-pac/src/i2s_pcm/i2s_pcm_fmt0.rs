#[doc = "Register `i2s_pcm_fmt0` reader"]
pub struct R(crate::R<I2S_PCM_FMT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_FMT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_FMT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_FMT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_fmt0` writer"]
pub struct W(crate::W<I2S_PCM_FMT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_FMT0_SPEC>;
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
impl From<crate::W<I2S_PCM_FMT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_FMT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sw` reader - Slot Width Select"]
pub type SW_R = crate::FieldReader<u8, SW_A>;
#[doc = "Slot Width Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "1: 8-bit"]
    BITS_8 = 1,
    #[doc = "2: 12-bit"]
    BITS_12 = 2,
    #[doc = "3: 16-bit"]
    BITS_16 = 3,
    #[doc = "4: 20-bit"]
    BITS_20 = 4,
    #[doc = "5: 24-bit"]
    BITS_24 = 5,
    #[doc = "6: 28-bit"]
    BITS_28 = 6,
    #[doc = "7: 32-bit"]
    BITS_32 = 7,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            1 => SW_A::BITS_8,
            2 => SW_A::BITS_12,
            3 => SW_A::BITS_16,
            4 => SW_A::BITS_20,
            5 => SW_A::BITS_24,
            6 => SW_A::BITS_28,
            7 => SW_A::BITS_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_8`"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == SW_A::BITS_8
    }
    #[doc = "Checks if the value of the field is `BITS_12`"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == SW_A::BITS_12
    }
    #[doc = "Checks if the value of the field is `BITS_16`"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == SW_A::BITS_16
    }
    #[doc = "Checks if the value of the field is `BITS_20`"]
    #[inline(always)]
    pub fn is_bits_20(&self) -> bool {
        *self == SW_A::BITS_20
    }
    #[doc = "Checks if the value of the field is `BITS_24`"]
    #[inline(always)]
    pub fn is_bits_24(&self) -> bool {
        *self == SW_A::BITS_24
    }
    #[doc = "Checks if the value of the field is `BITS_28`"]
    #[inline(always)]
    pub fn is_bits_28(&self) -> bool {
        *self == SW_A::BITS_28
    }
    #[doc = "Checks if the value of the field is `BITS_32`"]
    #[inline(always)]
    pub fn is_bits_32(&self) -> bool {
        *self == SW_A::BITS_32
    }
}
#[doc = "Field `sw` writer - Slot Width Select"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_PCM_FMT0_SPEC, u8, SW_A, 3, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut W {
        self.variant(SW_A::BITS_8)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut W {
        self.variant(SW_A::BITS_12)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut W {
        self.variant(SW_A::BITS_16)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bits_20(self) -> &'a mut W {
        self.variant(SW_A::BITS_20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bits_24(self) -> &'a mut W {
        self.variant(SW_A::BITS_24)
    }
    #[doc = "28-bit"]
    #[inline(always)]
    pub fn bits_28(self) -> &'a mut W {
        self.variant(SW_A::BITS_28)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bits_32(self) -> &'a mut W {
        self.variant(SW_A::BITS_32)
    }
}
#[doc = "Field `edge_transfer` reader - Edge Transfer"]
pub type EDGE_TRANSFER_R = crate::BitReader<EDGE_TRANSFER_A>;
#[doc = "Edge Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_TRANSFER_A {
    #[doc = "0: DOUT drives data and DIN sample data at alternate BCLK edge"]
    ALTERNATE = 0,
    #[doc = "1: DOUT drives data and DIN sample data at same BCLK edge"]
    SAME = 1,
}
impl From<EDGE_TRANSFER_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_TRANSFER_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_TRANSFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_TRANSFER_A {
        match self.bits {
            false => EDGE_TRANSFER_A::ALTERNATE,
            true => EDGE_TRANSFER_A::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == EDGE_TRANSFER_A::ALTERNATE
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == EDGE_TRANSFER_A::SAME
    }
}
#[doc = "Field `edge_transfer` writer - Edge Transfer"]
pub type EDGE_TRANSFER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_FMT0_SPEC, EDGE_TRANSFER_A, O>;
impl<'a, const O: u8> EDGE_TRANSFER_W<'a, O> {
    #[doc = "DOUT drives data and DIN sample data at alternate BCLK edge"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(EDGE_TRANSFER_A::ALTERNATE)
    }
    #[doc = "DOUT drives data and DIN sample data at same BCLK edge"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(EDGE_TRANSFER_A::SAME)
    }
}
#[doc = "Field `sr` reader - Sample Resolution"]
pub type SR_R = crate::FieldReader<u8, SR_A>;
#[doc = "Sample Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SR_A {
    #[doc = "1: 8-bit"]
    BITS_8 = 1,
    #[doc = "2: 12-bit"]
    BITS_12 = 2,
    #[doc = "3: 16-bit"]
    BITS_16 = 3,
    #[doc = "4: 20-bit"]
    BITS_20 = 4,
    #[doc = "5: 24-bit"]
    BITS_24 = 5,
    #[doc = "6: 28-bit"]
    BITS_28 = 6,
    #[doc = "7: 32-bit"]
    BITS_32 = 7,
}
impl From<SR_A> for u8 {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as _
    }
}
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            1 => SR_A::BITS_8,
            2 => SR_A::BITS_12,
            3 => SR_A::BITS_16,
            4 => SR_A::BITS_20,
            5 => SR_A::BITS_24,
            6 => SR_A::BITS_28,
            7 => SR_A::BITS_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS_8`"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == SR_A::BITS_8
    }
    #[doc = "Checks if the value of the field is `BITS_12`"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == SR_A::BITS_12
    }
    #[doc = "Checks if the value of the field is `BITS_16`"]
    #[inline(always)]
    pub fn is_bits_16(&self) -> bool {
        *self == SR_A::BITS_16
    }
    #[doc = "Checks if the value of the field is `BITS_20`"]
    #[inline(always)]
    pub fn is_bits_20(&self) -> bool {
        *self == SR_A::BITS_20
    }
    #[doc = "Checks if the value of the field is `BITS_24`"]
    #[inline(always)]
    pub fn is_bits_24(&self) -> bool {
        *self == SR_A::BITS_24
    }
    #[doc = "Checks if the value of the field is `BITS_28`"]
    #[inline(always)]
    pub fn is_bits_28(&self) -> bool {
        *self == SR_A::BITS_28
    }
    #[doc = "Checks if the value of the field is `BITS_32`"]
    #[inline(always)]
    pub fn is_bits_32(&self) -> bool {
        *self == SR_A::BITS_32
    }
}
#[doc = "Field `sr` writer - Sample Resolution"]
pub type SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_PCM_FMT0_SPEC, u8, SR_A, 3, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut W {
        self.variant(SR_A::BITS_8)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut W {
        self.variant(SR_A::BITS_12)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bits_16(self) -> &'a mut W {
        self.variant(SR_A::BITS_16)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bits_20(self) -> &'a mut W {
        self.variant(SR_A::BITS_20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bits_24(self) -> &'a mut W {
        self.variant(SR_A::BITS_24)
    }
    #[doc = "28-bit"]
    #[inline(always)]
    pub fn bits_28(self) -> &'a mut W {
        self.variant(SR_A::BITS_28)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bits_32(self) -> &'a mut W {
        self.variant(SR_A::BITS_32)
    }
}
#[doc = "Field `blck_polarity` reader - BCLK Polarity"]
pub type BLCK_POLARITY_R = crate::BitReader<BLCK_POLARITY_A>;
#[doc = "BCLK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLCK_POLARITY_A {
    #[doc = "0: DOUT drives data at negative edge"]
    NORMAL = 0,
    #[doc = "1: DOUT drives data at positive edge"]
    INVERT = 1,
}
impl From<BLCK_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: BLCK_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl BLCK_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLCK_POLARITY_A {
        match self.bits {
            false => BLCK_POLARITY_A::NORMAL,
            true => BLCK_POLARITY_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BLCK_POLARITY_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == BLCK_POLARITY_A::INVERT
    }
}
#[doc = "Field `blck_polarity` writer - BCLK Polarity"]
pub type BLCK_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_FMT0_SPEC, BLCK_POLARITY_A, O>;
impl<'a, const O: u8> BLCK_POLARITY_W<'a, O> {
    #[doc = "DOUT drives data at negative edge"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BLCK_POLARITY_A::NORMAL)
    }
    #[doc = "DOUT drives data at positive edge"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(BLCK_POLARITY_A::INVERT)
    }
}
#[doc = "Field `lrck_period` reader - Number of BCLKs per channel of the sample frame. This value is interpreted as follows:\n\nPCM mode: Number of BCLKs within (Left + Right) channel width.\n\nI2S/Left-justified/Right-justified: Number of BCLKs within each channel width (Left or Right)\n\nPeriod = N + 1\n\ne.g. N = 7: 8 BCLKs width"]
pub type LRCK_PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `lrck_period` writer - Number of BCLKs per channel of the sample frame. This value is interpreted as follows:\n\nPCM mode: Number of BCLKs within (Left + Right) channel width.\n\nI2S/Left-justified/Right-justified: Number of BCLKs within each channel width (Left or Right)\n\nPeriod = N + 1\n\ne.g. N = 7: 8 BCLKs width"]
pub type LRCK_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_FMT0_SPEC, u16, u16, 10, O>;
#[doc = "Field `lrck_polarity` reader - LRCK Polarity"]
pub type LRCK_POLARITY_R = crate::BitReader<LRCK_POLARITY_A>;
#[doc = "LRCK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCK_POLARITY_A {
    #[doc = "0: I2S/Left-justified/Right-justified: Left-channel when LRCK is low\n\nPCM: LRCK asserted at negative edge."]
    LOW = 0,
    #[doc = "1: I2S/LeftJustified/Right-justified: Left-channel when LRCK is high\n\nPCM: LRCK asserted at positive edge."]
    HIGH = 1,
}
impl From<LRCK_POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: LRCK_POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl LRCK_POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRCK_POLARITY_A {
        match self.bits {
            false => LRCK_POLARITY_A::LOW,
            true => LRCK_POLARITY_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LRCK_POLARITY_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LRCK_POLARITY_A::HIGH
    }
}
#[doc = "Field `lrck_polarity` writer - LRCK Polarity"]
pub type LRCK_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_FMT0_SPEC, LRCK_POLARITY_A, O>;
impl<'a, const O: u8> LRCK_POLARITY_W<'a, O> {
    #[doc = "I2S/Left-justified/Right-justified: Left-channel when LRCK is low\n\nPCM: LRCK asserted at negative edge."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LRCK_POLARITY_A::LOW)
    }
    #[doc = "I2S/LeftJustified/Right-justified: Left-channel when LRCK is high\n\nPCM: LRCK asserted at positive edge."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LRCK_POLARITY_A::HIGH)
    }
}
#[doc = "Field `lrck_width` reader - LRCK Width (only applies to the PCM mode)"]
pub type LRCK_WIDTH_R = crate::BitReader<LRCK_WIDTH_A>;
#[doc = "LRCK Width (only applies to the PCM mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRCK_WIDTH_A {
    #[doc = "0: LRCK = 1 BCLK Width"]
    SHORT = 0,
    #[doc = "1: LRCK = 2 BCLK Width"]
    LONG = 1,
}
impl From<LRCK_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: LRCK_WIDTH_A) -> Self {
        variant as u8 != 0
    }
}
impl LRCK_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRCK_WIDTH_A {
        match self.bits {
            false => LRCK_WIDTH_A::SHORT,
            true => LRCK_WIDTH_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == LRCK_WIDTH_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == LRCK_WIDTH_A::LONG
    }
}
#[doc = "Field `lrck_width` writer - LRCK Width (only applies to the PCM mode)"]
pub type LRCK_WIDTH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_PCM_FMT0_SPEC, LRCK_WIDTH_A, O>;
impl<'a, const O: u8> LRCK_WIDTH_W<'a, O> {
    #[doc = "LRCK = 1 BCLK Width"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(LRCK_WIDTH_A::SHORT)
    }
    #[doc = "LRCK = 2 BCLK Width"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(LRCK_WIDTH_A::LONG)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slot Width Select"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Edge Transfer"]
    #[inline(always)]
    pub fn edge_transfer(&self) -> EDGE_TRANSFER_R {
        EDGE_TRANSFER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Sample Resolution"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - BCLK Polarity"]
    #[inline(always)]
    pub fn blck_polarity(&self) -> BLCK_POLARITY_R {
        BLCK_POLARITY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - Number of BCLKs per channel of the sample frame. This value is interpreted as follows:\n\nPCM mode: Number of BCLKs within (Left + Right) channel width.\n\nI2S/Left-justified/Right-justified: Number of BCLKs within each channel width (Left or Right)\n\nPeriod = N + 1\n\ne.g. N = 7: 8 BCLKs width"]
    #[inline(always)]
    pub fn lrck_period(&self) -> LRCK_PERIOD_R {
        LRCK_PERIOD_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - LRCK Polarity"]
    #[inline(always)]
    pub fn lrck_polarity(&self) -> LRCK_POLARITY_R {
        LRCK_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - LRCK Width (only applies to the PCM mode)"]
    #[inline(always)]
    pub fn lrck_width(&self) -> LRCK_WIDTH_R {
        LRCK_WIDTH_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slot Width Select"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bit 3 - Edge Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn edge_transfer(&mut self) -> EDGE_TRANSFER_W<3> {
        EDGE_TRANSFER_W::new(self)
    }
    #[doc = "Bits 4:6 - Sample Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<4> {
        SR_W::new(self)
    }
    #[doc = "Bit 7 - BCLK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn blck_polarity(&mut self) -> BLCK_POLARITY_W<7> {
        BLCK_POLARITY_W::new(self)
    }
    #[doc = "Bits 8:17 - Number of BCLKs per channel of the sample frame. This value is interpreted as follows:\n\nPCM mode: Number of BCLKs within (Left + Right) channel width.\n\nI2S/Left-justified/Right-justified: Number of BCLKs within each channel width (Left or Right)\n\nPeriod = N + 1\n\ne.g. N = 7: 8 BCLKs width"]
    #[inline(always)]
    #[must_use]
    pub fn lrck_period(&mut self) -> LRCK_PERIOD_W<8> {
        LRCK_PERIOD_W::new(self)
    }
    #[doc = "Bit 19 - LRCK Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn lrck_polarity(&mut self) -> LRCK_POLARITY_W<19> {
        LRCK_POLARITY_W::new(self)
    }
    #[doc = "Bit 30 - LRCK Width (only applies to the PCM mode)"]
    #[inline(always)]
    #[must_use]
    pub fn lrck_width(&mut self) -> LRCK_WIDTH_W<30> {
        LRCK_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM Format Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_fmt0](index.html) module"]
pub struct I2S_PCM_FMT0_SPEC;
impl crate::RegisterSpec for I2S_PCM_FMT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_fmt0::R](R) reader structure"]
impl crate::Readable for I2S_PCM_FMT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_fmt0::W](W) writer structure"]
impl crate::Writable for I2S_PCM_FMT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_fmt0 to value 0"]
impl crate::Resettable for I2S_PCM_FMT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
