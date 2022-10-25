#[doc = "Register `i2s_pcm_tx3chmap1` reader"]
pub struct R(crate::R<I2S_PCM_TX3CHMAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_TX3CHMAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_TX3CHMAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_TX3CHMAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_tx3chmap1` writer"]
pub struct W(crate::W<I2S_PCM_TX3CHMAP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_TX3CHMAP1_SPEC>;
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
impl From<crate::W<I2S_PCM_TX3CHMAP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_TX3CHMAP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_map[0-7]` reader - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_map[0-7]` writer - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TX3CHMAP1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Channel [0-7] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub unsafe fn ch_map(&self, n: u8) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch0_map(&self) -> CH_MAP_R {
        CH_MAP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch1_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch2_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch3_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Channel 4 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch4_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Channel 5 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch5_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Channel 6 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch6_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Channel 7 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    pub fn ch7_map(&self) -> CH_MAP_R {
        CH_MAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Channel [0-7] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_map<const O: u8>(&mut self) -> CH_MAP_W<O> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 0:3 - Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_map(&mut self) -> CH_MAP_W<0> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_map(&mut self) -> CH_MAP_W<4> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 8:11 - Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_map(&mut self) -> CH_MAP_W<8> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_map(&mut self) -> CH_MAP_W<12> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 16:19 - Channel 4 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_map(&mut self) -> CH_MAP_W<16> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 20:23 - Channel 5 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_map(&mut self) -> CH_MAP_W<20> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 24:27 - Channel 6 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_map(&mut self) -> CH_MAP_W<24> {
        CH_MAP_W::new(self)
    }
    #[doc = "Bits 28:31 - Channel 7 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_map(&mut self) -> CH_MAP_W<28> {
        CH_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM TX3 Channel Mapping Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_tx3chmap1](index.html) module"]
pub struct I2S_PCM_TX3CHMAP1_SPEC;
impl crate::RegisterSpec for I2S_PCM_TX3CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_tx3chmap1::R](R) reader structure"]
impl crate::Readable for I2S_PCM_TX3CHMAP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_tx3chmap1::W](W) writer structure"]
impl crate::Writable for I2S_PCM_TX3CHMAP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_tx3chmap1 to value 0"]
impl crate::Resettable for I2S_PCM_TX3CHMAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
