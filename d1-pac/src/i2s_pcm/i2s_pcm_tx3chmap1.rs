#[doc = "Register `i2s_pcm_tx3chmap1` reader"]
pub type R = crate::R<I2S_PCM_TX3CHMAP1_SPEC>;
#[doc = "Register `i2s_pcm_tx3chmap1` writer"]
pub type W = crate::W<I2S_PCM_TX3CHMAP1_SPEC>;
#[doc = "Field `ch_map[0-7]` reader - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_R = crate::FieldReader;
#[doc = "Field `ch_map[0-7]` writer - Channel %s mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
pub type CH_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Channel [0-7] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_map` field"]
    #[inline(always)]
    pub fn ch_map(&self, n: u8) -> CH_MAP_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
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
    #[doc = "Channel [0-7] mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `ch0_map` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map(&mut self, n: u8) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_MAP_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - Channel 0 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel 1 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Channel 2 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Channel 3 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Channel 4 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Channel 5 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch5_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Channel 6 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch6_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Channel 7 mapping\n\n0000: The first sample\n\n...\n\n1111: The sixteenth sample"]
    #[inline(always)]
    #[must_use]
    pub fn ch7_map(&mut self) -> CH_MAP_W<I2S_PCM_TX3CHMAP1_SPEC> {
        CH_MAP_W::new(self, 28)
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
#[doc = "I2S/PCM TX3 Channel Mapping Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx3chmap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx3chmap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_TX3CHMAP1_SPEC;
impl crate::RegisterSpec for I2S_PCM_TX3CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_tx3chmap1::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_TX3CHMAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_tx3chmap1::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_TX3CHMAP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_tx3chmap1 to value 0"]
impl crate::Resettable for I2S_PCM_TX3CHMAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
