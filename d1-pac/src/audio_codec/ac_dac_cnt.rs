#[doc = "Register `ac_dac_cnt` reader"]
pub type R = crate::R<AC_DAC_CNT_SPEC>;
#[doc = "Register `ac_dac_cnt` writer"]
pub type W = crate::W<AC_DAC_CNT_SPEC>;
#[doc = "Field `tx_cnt` reader - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type TX_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `tx_cnt` writer - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type TX_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX Sample Counter\n\nThe audio sample number of sending into TXFIFO.\n\nWhen one sample is put into TXFIFO by DMA or by host IO, the TX sample counter register increases by one. The TX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cnt(&mut self) -> TX_CNT_W<AC_DAC_CNT_SPEC> {
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
#[doc = "DAC TX FIFO Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_CNT_SPEC;
impl crate::RegisterSpec for AC_DAC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_cnt::R`](R) reader structure"]
impl crate::Readable for AC_DAC_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_cnt::W`](W) writer structure"]
impl crate::Writable for AC_DAC_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_cnt to value 0"]
impl crate::Resettable for AC_DAC_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
