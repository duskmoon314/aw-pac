#[doc = "Register `i2s_pcm_tx2chsel` reader"]
pub struct R(crate::R<I2S_PCM_TX2CHSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_TX2CHSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_TX2CHSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_TX2CHSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_tx2chsel` writer"]
pub struct W(crate::W<I2S_PCM_TX2CHSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_TX2CHSEL_SPEC>;
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
impl From<crate::W<I2S_PCM_TX2CHSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_TX2CHSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chen` reader - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
pub type CHEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `chen` writer - TX Channel (Slot Enable)\n\nThe bit\\[15:0\\] refer to Slot \\[15:0\\]. When one or more slots are disabled, the affected slots are set to the disable state."]
pub type CHEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TX2CHSEL_SPEC, u16, u16, 16, O>;
#[doc = "Field `chsel` reader - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
pub type CHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chsel` writer - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
pub type CHSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TX2CHSEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `offset` reader - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
pub type OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `offset` writer - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
pub type OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TX2CHSEL_SPEC, u8, u8, 2, O>;
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
    pub fn chen(&mut self) -> CHEN_W<0> {
        CHEN_W::new(self)
    }
    #[doc = "Bits 16:19 - TX Channel (Slot) number select for each output\n\nNum channels = N + 1"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<16> {
        CHSEL_W::new(self)
    }
    #[doc = "Bits 20:21 - TX Offset Tune (TX Data offset to LRCK)\n\n0: No offset\n\nN: Data is offset by N BCLKs to LRCK"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<20> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM TX2 Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_tx2chsel](index.html) module"]
pub struct I2S_PCM_TX2CHSEL_SPEC;
impl crate::RegisterSpec for I2S_PCM_TX2CHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_tx2chsel::R](R) reader structure"]
impl crate::Readable for I2S_PCM_TX2CHSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_tx2chsel::W](W) writer structure"]
impl crate::Writable for I2S_PCM_TX2CHSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_tx2chsel to value 0"]
impl crate::Resettable for I2S_PCM_TX2CHSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
