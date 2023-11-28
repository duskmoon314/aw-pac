#[doc = "Register `i2s_pcm_txcnt` reader"]
pub type R = crate::R<I2S_PCM_TXCNT_SPEC>;
#[doc = "Register `i2s_pcm_txcnt` writer"]
pub type W = crate::W<I2S_PCM_TXCNT_SPEC>;
#[doc = "Field `tx_cnt` reader - TX Sample Counter"]
pub type TX_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `tx_cnt` writer - TX Sample Counter"]
pub type TX_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TX Sample Counter"]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Sample Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cnt(&mut self) -> TX_CNT_W<I2S_PCM_TXCNT_SPEC> {
        TX_CNT_W::new(self, 0)
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
#[doc = "I2S/PCM TX Sample Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_pcm_txcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_pcm_txcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_PCM_TXCNT_SPEC;
impl crate::RegisterSpec for I2S_PCM_TXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_pcm_txcnt::R`](R) reader structure"]
impl crate::Readable for I2S_PCM_TXCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_pcm_txcnt::W`](W) writer structure"]
impl crate::Writable for I2S_PCM_TXCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_pcm_txcnt to value 0"]
impl crate::Resettable for I2S_PCM_TXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
