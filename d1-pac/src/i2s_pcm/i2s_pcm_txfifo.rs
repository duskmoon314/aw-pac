#[doc = "Register `i2s_pcm_txfifo` reader"]
pub struct R(crate::R<I2S_PCM_TXFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_TXFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_TXFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_TXFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_txfifo` writer"]
pub struct W(crate::W<I2S_PCM_TXFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_TXFIFO_SPEC>;
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
impl From<crate::W<I2S_PCM_TXFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_TXFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `txdata` reader - TX Sample\n\nTransmitting left, right channel sample data should be written to this register one by one. The left channel sample data is first and then the right channel sample."]
pub type TXDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `txdata` writer - TX Sample\n\nTransmitting left, right channel sample data should be written to this register one by one. The left channel sample data is first and then the right channel sample."]
pub type TXDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_TXFIFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TX Sample\n\nTransmitting left, right channel sample data should be written to this register one by one. The left channel sample data is first and then the right channel sample."]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Sample\n\nTransmitting left, right channel sample data should be written to this register one by one. The left channel sample data is first and then the right channel sample."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM TXFIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_txfifo](index.html) module"]
pub struct I2S_PCM_TXFIFO_SPEC;
impl crate::RegisterSpec for I2S_PCM_TXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_txfifo::R](R) reader structure"]
impl crate::Readable for I2S_PCM_TXFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_txfifo::W](W) writer structure"]
impl crate::Writable for I2S_PCM_TXFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_txfifo to value 0"]
impl crate::Resettable for I2S_PCM_TXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
