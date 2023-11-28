#[doc = "Register `ac_dac_drc_hpflgain` reader"]
pub type R = crate::R<AC_DAC_DRC_HPFLGAIN_SPEC>;
#[doc = "Register `ac_dac_drc_hpflgain` writer"]
pub type W = crate::W<AC_DAC_DRC_HPFLGAIN_SPEC>;
#[doc = "Field `dac_drc_hpflgain` reader - The gain of HPF coefficient. The format is 3.24. (gain = 1)"]
pub type DAC_DRC_HPFLGAIN_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_hpflgain` writer - The gain of HPF coefficient. The format is 3.24. (gain = 1)"]
pub type DAC_DRC_HPFLGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The gain of HPF coefficient. The format is 3.24. (gain = 1)"]
    #[inline(always)]
    pub fn dac_drc_hpflgain(&self) -> DAC_DRC_HPFLGAIN_R {
        DAC_DRC_HPFLGAIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The gain of HPF coefficient. The format is 3.24. (gain = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_hpflgain(&mut self) -> DAC_DRC_HPFLGAIN_W<AC_DAC_DRC_HPFLGAIN_SPEC> {
        DAC_DRC_HPFLGAIN_W::new(self, 0)
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
#[doc = "DAC DRC HPF Gain Low Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hpflgain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hpflgain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HPFLGAIN_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HPFLGAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_hpflgain::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HPFLGAIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_hpflgain::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HPFLGAIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hpflgain to value 0"]
impl crate::Resettable for AC_DAC_DRC_HPFLGAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
