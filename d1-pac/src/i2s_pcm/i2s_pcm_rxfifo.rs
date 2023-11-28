#[doc = "Register `i2s_pcm_rxfifo` reader"]
pub type R = crate::R<I2S_PCM_RXFIFO_SPEC>;
#[doc = "Register `i2s_pcm_rxfifo` writer"]
pub type W = crate::W<I2S_PCM_RXFIFO_SPEC>;
#[doc = "Field `rx_data` reader - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data is first and then the right channel sample."]
pub type RX_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `rx_data` writer - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data is first and then the right channel sample."]
pub type RX_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data is first and then the right channel sample."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data is first and then the right channel sample."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data(&mut self) -> RX_DATA_W<I2S_PCM_RXFIFO_SPEC> {
        RX_DATA_W::new(self, 0)
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
#[doc = "I2S/PCM RXFIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_rxfifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_rxfifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_RXFIFO_SPEC;
impl crate::RegisterSpec for I2S_PCM_RXFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_rxfifo::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_RXFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_rxfifo::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_RXFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_rxfifo to value 0"]
impl crate::Resettable for I2S_PCM_RXFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
