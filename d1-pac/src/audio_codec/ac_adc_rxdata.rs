#[doc = "Register `ac_adc_rxdata` reader"]
pub type R = crate::R<AC_ADC_RXDATA_SPEC>;
#[doc = "Register `ac_adc_rxdata` writer"]
pub type W = crate::W<AC_ADC_RXDATA_SPEC>;
#[doc = "Field `rx_data` reader - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data comes first and then the right channel sample."]
pub type RX_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX Sample\n\nThe host can get one sample by reading this register. The left channel sample data comes first and then the right channel sample."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(self.bits)
    }
}
impl W {
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
#[doc = "ADC RX Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_rxdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_rxdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_RXDATA_SPEC;
impl crate::RegisterSpec for AC_ADC_RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_rxdata::R`](R) reader structure"]
impl crate::Readable for AC_ADC_RXDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_rxdata::W`](W) writer structure"]
impl crate::Writable for AC_ADC_RXDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_rxdata to value 0"]
impl crate::Resettable for AC_ADC_RXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
