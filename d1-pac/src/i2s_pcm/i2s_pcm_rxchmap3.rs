#[doc = "Register `i2s_pcm_rxchmap3` reader"]
pub struct R(crate::R<I2S_PCM_RXCHMAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_RXCHMAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_RXCHMAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_RXCHMAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_rxchmap3` writer"]
pub struct W(crate::W<I2S_PCM_RXCHMAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_RXCHMAP3_SPEC>;
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
impl From<crate::W<I2S_PCM_RXCHMAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_RXCHMAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_map[0-3]` reader - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_map[0-3]` writer - RX Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_RXCHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `ch_select[0-3]` reader - RX Channel %s Select"]
pub type CH_SELECT_R = crate::FieldReader<u8, CH_SELECT_A>;
#[doc = "RX Channel %s Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH_SELECT_A {
    #[doc = "0: `0`"]
    SDI0 = 0,
    #[doc = "1: `1`"]
    SDI1 = 1,
    #[doc = "2: `10`"]
    SDI2 = 2,
    #[doc = "3: `11`"]
    SDI3 = 3,
}
impl From<CH_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_SELECT_A) -> Self {
        variant as _
    }
}
impl CH_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_SELECT_A {
        match self.bits {
            0 => CH_SELECT_A::SDI0,
            1 => CH_SELECT_A::SDI1,
            2 => CH_SELECT_A::SDI2,
            3 => CH_SELECT_A::SDI3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SDI0`"]
    #[inline(always)]
    pub fn is_sdi0(&self) -> bool {
        *self == CH_SELECT_A::SDI0
    }
    #[doc = "Checks if the value of the field is `SDI1`"]
    #[inline(always)]
    pub fn is_sdi1(&self) -> bool {
        *self == CH_SELECT_A::SDI1
    }
    #[doc = "Checks if the value of the field is `SDI2`"]
    #[inline(always)]
    pub fn is_sdi2(&self) -> bool {
        *self == CH_SELECT_A::SDI2
    }
    #[doc = "Checks if the value of the field is `SDI3`"]
    #[inline(always)]
    pub fn is_sdi3(&self) -> bool {
        *self == CH_SELECT_A::SDI3
    }
}
#[doc = "Field `ch_select[0-3]` writer - RX Channel %s Select"]
pub type CH_SELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, I2S_PCM_RXCHMAP3_SPEC, u8, CH_SELECT_A, 2, O>;
impl<'a, const O: u8> CH_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdi0(self) -> &'a mut W {
        self.variant(CH_SELECT_A::SDI0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdi1(self) -> &'a mut W {
        self.variant(CH_SELECT_A::SDI1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sdi2(self) -> &'a mut W {
        self.variant(CH_SELECT_A::SDI2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn sdi3(self) -> &'a mut W {
        self.variant(CH_SELECT_A::SDI3)
    }
}
impl R {
    #[doc = "RX Channel [0-3] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub unsafe fn ch_map(&self, n: u8) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - RX Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch0_map(&self) -> CH_MAP_R {
        CH_MAP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch1_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RX Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch2_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RX Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch3_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "RX Channel [0-3] Select"]
    #[inline(always)]
    pub unsafe fn ch_select(&self, n: u8) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> (n * 8 + 4)) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RX Channel 0 Select"]
    #[inline(always)]
    pub fn ch0_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - RX Channel 1 Select"]
    #[inline(always)]
    pub fn ch1_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RX Channel 2 Select"]
    #[inline(always)]
    pub fn ch2_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - RX Channel 3 Select"]
    #[inline(always)]
    pub fn ch3_select(&self) -> CH_SELECT_R {
        CH_SELECT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "RX Channel [0-3] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_map<const O: u8>(&mut self) -> CH_MAP_W<O> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 0:3 - RX Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_map(&mut self) -> CH_MAP_W<0> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 8:11 - RX Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_map(&mut self) -> CH_MAP_W<8> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 16:19 - RX Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_map(&mut self) -> CH_MAP_W<16> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 24:27 - RX Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_map(&mut self) -> CH_MAP_W<24> {
        CH_MAP_W::new(self)
    }
    #[doc = "RX Channel [0-3] Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_select<const O: u8>(&mut self) -> CH_SELECT_W<O> {
        CH_SELECT_W::new(self)
    }
    #[doc = "Bits 4:5 - RX Channel 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_select(&mut self) -> CH_SELECT_W<4> {
        CH_SELECT_W::new(self)
    }
    #[doc = "Bits 12:13 - RX Channel 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_select(&mut self) -> CH_SELECT_W<12> {
        CH_SELECT_W::new(self)
    }
    #[doc = "Bits 20:21 - RX Channel 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_select(&mut self) -> CH_SELECT_W<20> {
        CH_SELECT_W::new(self)
    }
    #[doc = "Bits 28:29 - RX Channel 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_select(&mut self) -> CH_SELECT_W<28> {
        CH_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM RX Channel Mapping Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_rxchmap3](index.html) module"]
pub struct I2S_PCM_RXCHMAP3_SPEC;
impl crate::RegisterSpec for I2S_PCM_RXCHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_rxchmap3::R](R) reader structure"]
impl crate::Readable for I2S_PCM_RXCHMAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_rxchmap3::W](W) writer structure"]
impl crate::Writable for I2S_PCM_RXCHMAP3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_rxchmap3 to value 0"]
impl crate::Resettable for I2S_PCM_RXCHMAP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
