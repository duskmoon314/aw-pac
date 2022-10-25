#[doc = "Register `i2s_pcm_tx3chmap0` reader"]
pub struct R(crate::R<I2S_PCM_TX3CHMAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_TX3CHMAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_TX3CHMAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_TX3CHMAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_tx3chmap0` writer"]
pub struct W(crate::W<I2S_PCM_TX3CHMAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_TX3CHMAP0_SPEC>;
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
impl From<crate::W<I2S_PCM_TX3CHMAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_TX3CHMAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_map[8-15]` reader - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_map[8-15]` writer - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TX3CHMAP0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Channel [8-15] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub unsafe fn ch_map(&self, n: u8) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> ((n - 8) * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Channel 8 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch8_map(&self) -> CH_MAP_R {
        CH_MAP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 9 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch9_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 10 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch10_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 11 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch11_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 12 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch12_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 13 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch13_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 14 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch14_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 15 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch15_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Channel [8-15] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_map<const O: u8>(&mut self) -> CH_MAP_W<O> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 0:3 - Channel 8 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch8_map(&mut self) -> CH_MAP_W<0> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 9 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch9_map(&mut self) -> CH_MAP_W<4> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 8:11 - Channel 10 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch10_map(&mut self) -> CH_MAP_W<8> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 11 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch11_map(&mut self) -> CH_MAP_W<12> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 16:19 - Channel 12 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch12_map(&mut self) -> CH_MAP_W<16> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 20:23 - Channel 13 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch13_map(&mut self) -> CH_MAP_W<20> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 24:27 - Channel 14 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch14_map(&mut self) -> CH_MAP_W<24> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 28:31 - Channel 15 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch15_map(&mut self) -> CH_MAP_W<28> {
        CH_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM TX3 Channel Mapping Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_tx3chmap0](index.html) module"]
pub struct I2S_PCM_TX3CHMAP0_SPEC;
impl crate::RegisterSpec for I2S_PCM_TX3CHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_tx3chmap0::R](R) reader structure"]
impl crate::Readable for I2S_PCM_TX3CHMAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_tx3chmap0::W](W) writer structure"]
impl crate::Writable for I2S_PCM_TX3CHMAP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_tx3chmap0 to value 0"]
impl crate::Resettable for I2S_PCM_TX3CHMAP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
