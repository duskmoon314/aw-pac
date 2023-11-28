#[doc = "Register `ac_adc_cnt` reader"]
pub type R = crate::R<AC_ADC_CNT_SPEC>;
#[doc = "Register `ac_adc_cnt` writer"]
pub type W = crate::W<AC_ADC_CNT_SPEC>;
#[doc = "Field `rx_cnt` reader - RX Sample Counter\n\nThe audio sample number of writing into RXFIFO. When one sample is written by Digital Audio Engine, the RX sample counter register increases by one. The RX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type RX_CNT_R = crate::FieldReader<u32>;
#[doc = "Field `rx_cnt` writer - RX Sample Counter\n\nThe audio sample number of writing into RXFIFO. When one sample is written by Digital Audio Engine, the RX sample counter register increases by one. The RX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
pub type RX_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX Sample Counter\n\nThe audio sample number of writing into RXFIFO. When one sample is written by Digital Audio Engine, the RX sample counter register increases by one. The RX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX Sample Counter\n\nThe audio sample number of writing into RXFIFO. When one sample is written by Digital Audio Engine, the RX sample counter register increases by one. The RX sample counter register can be set to any initial valve at any time. After being updated by the initial value, the counter register should count from this initial value."]
    #[inline(always)]
    #[must_use]
    pub fn rx_cnt(&mut self) -> RX_CNT_W<AC_ADC_CNT_SPEC> {
        RX_CNT_W::new(self, 0)
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
#[doc = "ADC RX Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_CNT_SPEC;
impl crate::RegisterSpec for AC_ADC_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_cnt::R`](R) reader structure"]
impl crate::Readable for AC_ADC_CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_cnt::W`](W) writer structure"]
impl crate::Writable for AC_ADC_CNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_cnt to value 0"]
impl crate::Resettable for AC_ADC_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
