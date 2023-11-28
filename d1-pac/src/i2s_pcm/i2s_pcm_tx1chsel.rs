#[doc = "Register `i2s_pcm_tx1chsel` reader"]
pub type R = crate::R<I2S_PCM_TX1CHSEL_SPEC>;
#[doc = "Register `i2s_pcm_tx1chsel` writer"]
pub type W = crate::W<I2S_PCM_TX1CHSEL_SPEC>;
#[doc = "Field `chen` reader - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
pub type CHEN_R = crate::FieldReader<u16>;
#[doc = "Field `chen` writer - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
pub type CHEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `chsel` reader - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
pub type CHSEL_R = crate::FieldReader;
#[doc = "Field `chsel` writer - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `offset` reader - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
pub type OFFSET_R = crate::FieldReader;
#[doc = "Field `offset` writer - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
pub type OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<I2S_PCM_TX1CHSEL_SPEC> {
        CHEN_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<I2S_PCM_TX1CHSEL_SPEC> {
        CHSEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<I2S_PCM_TX1CHSEL_SPEC> {
        OFFSET_W::new(self, 20)
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
#[doc = "I2S/PCM TX1 Channel Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_tx1chsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_tx1chsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_TX1CHSEL_SPEC;
impl crate::RegisterSpec for I2S_PCM_TX1CHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_tx1chsel::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_TX1CHSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_tx1chsel::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_TX1CHSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_tx1chsel to value 0"]
impl crate::Resettable for I2S_PCM_TX1CHSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
