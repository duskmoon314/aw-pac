#[doc = "Register `ac_adc_drc_mnghs` reader"]
pub type R = crate::R<AC_ADC_DRC_MNGHS_SPEC>;
#[doc = "Register `ac_adc_drc_mnghs` writer"]
pub type W = crate::W<AC_ADC_DRC_MNGHS_SPEC>;
#[doc = "Field `adc_drc_mnghs` reader - The min gain setting, which is determined by equation MXG =MXG/6.0206. The format is 8.24 and must -60 dB ≤ MNG ≤ -40 dB (The default value is -40 dB)"]
pub type ADC_DRC_MNGHS_R = crate::FieldReader<u16>;
#[doc = "Field `adc_drc_mnghs` writer - The min gain setting, which is determined by equation MXG =MXG/6.0206. The format is 8.24 and must -60 dB ≤ MNG ≤ -40 dB (The default value is -40 dB)"]
pub type ADC_DRC_MNGHS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The min gain setting, which is determined by equation MXG =MXG/6.0206. The format is 8.24 and must -60 dB ≤ MNG ≤ -40 dB (The default value is -40 dB)"]
    #[inline(always)]
    pub fn adc_drc_mnghs(&self) -> ADC_DRC_MNGHS_R {
        ADC_DRC_MNGHS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The min gain setting, which is determined by equation MXG =MXG/6.0206. The format is 8.24 and must -60 dB ≤ MNG ≤ -40 dB (The default value is -40 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn adc_drc_mnghs(&mut self) -> ADC_DRC_MNGHS_W<AC_ADC_DRC_MNGHS_SPEC> {
        ADC_DRC_MNGHS_W::new(self, 0)
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
#[doc = "ADC DRC MIN Gain High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_adc_drc_mnghs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_adc_drc_mnghs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_ADC_DRC_MNGHS_SPEC;
impl crate::RegisterSpec for AC_ADC_DRC_MNGHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_adc_drc_mnghs::R`](R) reader structure"]
impl crate::Readable for AC_ADC_DRC_MNGHS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_adc_drc_mnghs::W`](W) writer structure"]
impl crate::Writable for AC_ADC_DRC_MNGHS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_adc_drc_mnghs to value 0xf95b"]
impl crate::Resettable for AC_ADC_DRC_MNGHS_SPEC {
    const RESET_VALUE: Self::Ux = 0xf95b;
}
