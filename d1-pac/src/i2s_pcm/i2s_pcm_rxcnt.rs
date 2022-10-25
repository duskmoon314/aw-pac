#[doc = "Register `i2s_pcm_rxcnt` reader"]
pub struct R(crate::R<I2S_PCM_RXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_PCM_RXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_PCM_RXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_PCM_RXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_pcm_rxcnt` writer"]
pub struct W(crate::W<I2S_PCM_RXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_PCM_RXCNT_SPEC>;
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
impl From<crate::W<I2S_PCM_RXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_PCM_RXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_cnt` reader - RX Sample Counter"]
pub type RX_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rx_cnt` writer - RX Sample Counter"]
pub type RX_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_PCM_RXCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - RX Sample Counter"]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Sample Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cnt(&mut self) -> RX_CNT_W<0> {
        RX_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S/PCM RX Sample Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_pcm_rxcnt](index.html) module"]
pub struct I2S_PCM_RXCNT_SPEC;
impl crate::RegisterSpec for I2S_PCM_RXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_pcm_rxcnt::R](R) reader structure"]
impl crate::Readable for I2S_PCM_RXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_pcm_rxcnt::W](W) writer structure"]
impl crate::Writable for I2S_PCM_RXCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_rxcnt to value 0"]
impl crate::Resettable for I2S_PCM_RXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
