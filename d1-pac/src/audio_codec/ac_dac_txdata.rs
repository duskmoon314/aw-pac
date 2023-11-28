#[doc = "Register `ac_dac_txdata` writer"]
pub type W = crate::W<AC_DAC_TXDATA_SPEC>;
#[doc = "Field `tx_data` writer - Write the transmitting left and right channel sample data to this register one by one. Write the left channel sample data first and then the right channel sample."]
pub type TX_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write the transmitting left and right channel sample data to this register one by one. Write the left channel sample data first and then the right channel sample."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<AC_DAC_TXDATA_SPEC> {
        TX_DATA_W::new(self, 0)
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
#[doc = "DAC TX DATA Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_txdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_TXDATA_SPEC;
impl crate::RegisterSpec for AC_DAC_TXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ac_dac_txdata::W`](W) writer structure"]
impl crate::Writable for AC_DAC_TXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_txdata to value 0"]
impl crate::Resettable for AC_DAC_TXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
