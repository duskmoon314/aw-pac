#[doc = "Register `i2s_pcm_clkd` reader"]
pub struct R(crate::R<I2S_PCM_CLKD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_CLKD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_CLKD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_CLKD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_clkd` writer"]
pub struct W(crate::W<I2S_PCM_CLKD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_CLKD_SPEC>;
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
impl From<crate::W<I2S_PCM_CLKD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_CLKD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mclkdiv` reader - MCLK Divide ratio from PLL_AUDIO"]
pub type MCLKDIV_R = crate::FieldReader<u8, MCLKDIV_A>;
#[doc = "MCLK Divide ratio from PLL_AUDIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCLKDIV_A {
    #[doc = "1: Divide by 1"]
    DIV_1 = 1,
    #[doc = "2: Divide by 2"]
    DIV_2 = 2,
    #[doc = "3: Divide by 4"]
    DIV_4 = 3,
    #[doc = "4: Divide by 6"]
    DIV_6 = 4,
    #[doc = "5: Divide by 8"]
    DIV_8 = 5,
    #[doc = "6: Divide by 12"]
    DIV_12 = 6,
    #[doc = "7: Divide by 16"]
    DIV_16 = 7,
    #[doc = "8: Divide by 24"]
    DIV_24 = 8,
    #[doc = "9: Divide by 32"]
    DIV_32 = 9,
    #[doc = "10: Divide by 48"]
    DIV_48 = 10,
    #[doc = "11: Divide by 64"]
    DIV_64 = 11,
    #[doc = "12: Divide by 96"]
    DIV_96 = 12,
    #[doc = "13: Divide by 128"]
    DIV_128 = 13,
    #[doc = "14: Divide by 176"]
    DIV_176 = 14,
    #[doc = "15: Divide by 192"]
    DIV_192 = 15,
}
impl From<MCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKDIV_A) -> Self {
        variant as _
    }
}
impl MCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKDIV_A {
        match self.bits {
            1 => MCLKDIV_A::DIV_1,
            2 => MCLKDIV_A::DIV_2,
            3 => MCLKDIV_A::DIV_4,
            4 => MCLKDIV_A::DIV_6,
            5 => MCLKDIV_A::DIV_8,
            6 => MCLKDIV_A::DIV_12,
            7 => MCLKDIV_A::DIV_16,
            8 => MCLKDIV_A::DIV_24,
            9 => MCLKDIV_A::DIV_32,
            10 => MCLKDIV_A::DIV_48,
            11 => MCLKDIV_A::DIV_64,
            12 => MCLKDIV_A::DIV_96,
            13 => MCLKDIV_A::DIV_128,
            14 => MCLKDIV_A::DIV_176,
            15 => MCLKDIV_A::DIV_192,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == MCLKDIV_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == MCLKDIV_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == MCLKDIV_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_6`"]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == MCLKDIV_A::DIV_6
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == MCLKDIV_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_12`"]
    #[inline(always)]
    pub fn is_div_12(&self) -> bool {
        *self == MCLKDIV_A::DIV_12
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == MCLKDIV_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_24`"]
    #[inline(always)]
    pub fn is_div_24(&self) -> bool {
        *self == MCLKDIV_A::DIV_24
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == MCLKDIV_A::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_48`"]
    #[inline(always)]
    pub fn is_div_48(&self) -> bool {
        *self == MCLKDIV_A::DIV_48
    }
    #[doc = "Checks if the value of the field is `DIV_64`"]
    #[inline(always)]
    pub fn is_div_64(&self) -> bool {
        *self == MCLKDIV_A::DIV_64
    }
    #[doc = "Checks if the value of the field is `DIV_96`"]
    #[inline(always)]
    pub fn is_div_96(&self) -> bool {
        *self == MCLKDIV_A::DIV_96
    }
    #[doc = "Checks if the value of the field is `DIV_128`"]
    #[inline(always)]
    pub fn is_div_128(&self) -> bool {
        *self == MCLKDIV_A::DIV_128
    }
    #[doc = "Checks if the value of the field is `DIV_176`"]
    #[inline(always)]
    pub fn is_div_176(&self) -> bool {
        *self == MCLKDIV_A::DIV_176
    }
    #[doc = "Checks if the value of the field is `DIV_192`"]
    #[inline(always)]
    pub fn is_div_192(&self) -> bool {
        *self == MCLKDIV_A::DIV_192
    }
}
#[doc = "Field `mclkdiv` writer - MCLK Divide ratio from PLL_AUDIO"]
pub type MCLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_CLKD_SPEC, u8, MCLKDIV_A, 4, O>;
impl<'a, const O: u8> MCLKDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_8)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div_12(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_12)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_16)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn div_24(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_24)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_32)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn div_48(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_48)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div_64(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_64)
    }
    #[doc = "Divide by 96"]
    #[inline(always)]
    pub fn div_96(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_96)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div_128(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_128)
    }
    #[doc = "Divide by 176"]
    #[inline(always)]
    pub fn div_176(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_176)
    }
    #[doc = "Divide by 192"]
    #[inline(always)]
    pub fn div_192(self) -> &'a mut W {
        self.variant(MCLKDIV_A::DIV_192)
    }
}
#[doc = "Field `bclkdiv` reader - BCLK Divide ratio from PLL_AUDIO"]
pub type BCLKDIV_R = crate::FieldReader<u8, BCLKDIV_A>;
#[doc = "BCLK Divide ratio from PLL_AUDIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCLKDIV_A {
    #[doc = "1: Divide by 1"]
    DIV_1 = 1,
    #[doc = "2: Divide by 2"]
    DIV_2 = 2,
    #[doc = "3: Divide by 4"]
    DIV_4 = 3,
    #[doc = "4: Divide by 6"]
    DIV_6 = 4,
    #[doc = "5: Divide by 8"]
    DIV_8 = 5,
    #[doc = "6: Divide by 12"]
    DIV_12 = 6,
    #[doc = "7: Divide by 16"]
    DIV_16 = 7,
    #[doc = "8: Divide by 24"]
    DIV_24 = 8,
    #[doc = "9: Divide by 32"]
    DIV_32 = 9,
    #[doc = "10: Divide by 48"]
    DIV_48 = 10,
    #[doc = "11: Divide by 64"]
    DIV_64 = 11,
    #[doc = "12: Divide by 96"]
    DIV_96 = 12,
    #[doc = "13: Divide by 128"]
    DIV_128 = 13,
    #[doc = "14: Divide by 176"]
    DIV_176 = 14,
    #[doc = "15: Divide by 192"]
    DIV_192 = 15,
}
impl From<BCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: BCLKDIV_A) -> Self {
        variant as _
    }
}
impl BCLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCLKDIV_A {
        match self.bits {
            1 => BCLKDIV_A::DIV_1,
            2 => BCLKDIV_A::DIV_2,
            3 => BCLKDIV_A::DIV_4,
            4 => BCLKDIV_A::DIV_6,
            5 => BCLKDIV_A::DIV_8,
            6 => BCLKDIV_A::DIV_12,
            7 => BCLKDIV_A::DIV_16,
            8 => BCLKDIV_A::DIV_24,
            9 => BCLKDIV_A::DIV_32,
            10 => BCLKDIV_A::DIV_48,
            11 => BCLKDIV_A::DIV_64,
            12 => BCLKDIV_A::DIV_96,
            13 => BCLKDIV_A::DIV_128,
            14 => BCLKDIV_A::DIV_176,
            15 => BCLKDIV_A::DIV_192,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == BCLKDIV_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == BCLKDIV_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == BCLKDIV_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_6`"]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == BCLKDIV_A::DIV_6
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == BCLKDIV_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_12`"]
    #[inline(always)]
    pub fn is_div_12(&self) -> bool {
        *self == BCLKDIV_A::DIV_12
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == BCLKDIV_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_24`"]
    #[inline(always)]
    pub fn is_div_24(&self) -> bool {
        *self == BCLKDIV_A::DIV_24
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == BCLKDIV_A::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_48`"]
    #[inline(always)]
    pub fn is_div_48(&self) -> bool {
        *self == BCLKDIV_A::DIV_48
    }
    #[doc = "Checks if the value of the field is `DIV_64`"]
    #[inline(always)]
    pub fn is_div_64(&self) -> bool {
        *self == BCLKDIV_A::DIV_64
    }
    #[doc = "Checks if the value of the field is `DIV_96`"]
    #[inline(always)]
    pub fn is_div_96(&self) -> bool {
        *self == BCLKDIV_A::DIV_96
    }
    #[doc = "Checks if the value of the field is `DIV_128`"]
    #[inline(always)]
    pub fn is_div_128(&self) -> bool {
        *self == BCLKDIV_A::DIV_128
    }
    #[doc = "Checks if the value of the field is `DIV_176`"]
    #[inline(always)]
    pub fn is_div_176(&self) -> bool {
        *self == BCLKDIV_A::DIV_176
    }
    #[doc = "Checks if the value of the field is `DIV_192`"]
    #[inline(always)]
    pub fn is_div_192(&self) -> bool {
        *self == BCLKDIV_A::DIV_192
    }
}
#[doc = "Field `bclkdiv` writer - BCLK Divide ratio from PLL_AUDIO"]
pub type BCLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_CLKD_SPEC, u8, BCLKDIV_A, 4, O>;
impl<'a, const O: u8> BCLKDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_8)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn div_12(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_12)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_16)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn div_24(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_24)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_32)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn div_48(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_48)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div_64(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_64)
    }
    #[doc = "Divide by 96"]
    #[inline(always)]
    pub fn div_96(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_96)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div_128(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_128)
    }
    #[doc = "Divide by 176"]
    #[inline(always)]
    pub fn div_176(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_176)
    }
    #[doc = "Divide by 192"]
    #[inline(always)]
    pub fn div_192(self) -> &'a mut W {
        self.variant(BCLKDIV_A::DIV_192)
    }
}
#[doc = "Field `mclko_en` reader - MCLK Output Enable\n\nNote: Whether in slave or master mode, when this bit is set to ‘1’, MCLK should be output."]
pub type MCLKO_EN_R = crate::BitReader<MCLKO_EN_A>;
#[doc = "MCLK Output Enable\n\nNote: Whether in slave or master mode, when this bit is set to ‘1’, MCLK should be output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLKO_EN_A {
    #[doc = "0: Disable MCLK Output"]
    DISABLE = 0,
    #[doc = "1: Enable MCLK Output"]
    ENABLE = 1,
}
impl From<MCLKO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLKO_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCLKO_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKO_EN_A {
        match self.bits {
            false => MCLKO_EN_A::DISABLE,
            true => MCLKO_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MCLKO_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MCLKO_EN_A::ENABLE
    }
}
#[doc = "Field `mclko_en` writer - MCLK Output Enable\n\nNote: Whether in slave or master mode, when this bit is set to ‘1’, MCLK should be output."]
pub type MCLKO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_PCM_CLKD_SPEC, MCLKO_EN_A, O>;
impl<'a, const O: u8> MCLKO_EN_W<'a, O> {
    #[doc = "Disable MCLK Output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCLKO_EN_A::DISABLE)
    }
    #[doc = "Enable MCLK Output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCLKO_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - MCLK Divide ratio from PLL_AUDIO"]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - BCLK Divide ratio from PLL_AUDIO"]
    #[inline(always)]
    pub fn bclkdiv(&self) -> BCLKDIV_R {
        BCLKDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - MCLK Output Enable\n\nNote: Whether in slave or master mode, when this bit is set to ‘1’, MCLK should be output."]
    #[inline(always)]
    pub fn mclko_en(&self) -> MCLKO_EN_R {
        MCLKO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCLK Divide ratio from PLL_AUDIO"]
    #[inline(always)]
    #[must_use]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W<0> {
        MCLKDIV_W::new(self)
    }
    #[doc = "Bits 4:7 - BCLK Divide ratio from PLL_AUDIO"]
    #[inline(always)]
    #[must_use]
    pub fn bclkdiv(&mut self) -> BCLKDIV_W<4> {
        BCLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - MCLK Output Enable\n\nNote: Whether in slave or master mode, when this bit is set to ‘1’, MCLK should be output."]
    #[inline(always)]
    #[must_use]
    pub fn mclko_en(&mut self) -> MCLKO_EN_W<8> {
        MCLKO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM Clock Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_clkd](index.html) module"]
pub struct I2S_PCM_CLKD_SPEC;
impl crate::RegisterSpec for I2S_PCM_CLKD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_clkd::R](R) reader structure"]
impl crate::Readable for I2S_PCM_CLKD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_clkd::W](W) writer structure"]
impl crate::Writable for I2S_PCM_CLKD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_clkd to value 0"]
impl crate::Resettable for I2S_PCM_CLKD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
