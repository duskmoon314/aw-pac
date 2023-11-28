#[doc = "Register `ac_dac_drc_het` reader"]
pub type R = crate::R<AC_DAC_DRC_HET_SPEC>;
#[doc = "Register `ac_dac_drc_het` writer"]
pub type W = crate::W<AC_DAC_DRC_HET_SPEC>;
#[doc = "Field `dac_drc_het` reader - The expander threshold setting, which is set by the equation that ETin = -ET/6.0206. The format is 8.24. (The default value is -70 dB)"]
pub type DAC_DRC_HET_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_het` writer - The expander threshold setting, which is set by the equation that ETin = -ET/6.0206. The format is 8.24. (The default value is -70 dB)"]
pub type DAC_DRC_HET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The expander threshold setting, which is set by the equation that ETin = -ET/6.0206. The format is 8.24. (The default value is -70 dB)"]
    #[inline(always)]
    pub fn dac_drc_het(&self) -> DAC_DRC_HET_R {
        DAC_DRC_HET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The expander threshold setting, which is set by the equation that ETin = -ET/6.0206. The format is 8.24. (The default value is -70 dB)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_het(&mut self) -> DAC_DRC_HET_W<AC_DAC_DRC_HET_SPEC> {
        DAC_DRC_HET_W::new(self, 0)
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
#[doc = "DAC DRC Expander Threshold High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_het::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_het::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HET_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_het::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_het::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_het to value 0x0ba0"]
impl crate::Resettable for AC_DAC_DRC_HET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ba0;
}
