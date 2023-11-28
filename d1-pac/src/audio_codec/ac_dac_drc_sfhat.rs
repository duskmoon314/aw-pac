#[doc = "Register `ac_dac_drc_sfhat` reader"]
pub type R = crate::R<AC_DAC_DRC_SFHAT_SPEC>;
#[doc = "Register `ac_dac_drc_sfhat` writer"]
pub type W = crate::W<AC_DAC_DRC_SFHAT_SPEC>;
#[doc = "Field `dac_drc_sfhat` reader - The smooth filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 5 ms)"]
pub type DAC_DRC_SFHAT_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_sfhat` writer - The smooth filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 5 ms)"]
pub type DAC_DRC_SFHAT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - The smooth filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 5 ms)"]
    #[inline(always)]
    pub fn dac_drc_sfhat(&self) -> DAC_DRC_SFHAT_R {
        DAC_DRC_SFHAT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The smooth filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/tr). The format is 3.24. (The default value is 5 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_sfhat(&mut self) -> DAC_DRC_SFHAT_W<AC_DAC_DRC_SFHAT_SPEC> {
        DAC_DRC_SFHAT_W::new(self, 0)
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
#[doc = "DAC DRC Smooth filter Gain High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_sfhat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_sfhat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_SFHAT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_SFHAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_sfhat::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_SFHAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_sfhat::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_SFHAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_sfhat to value 0x02"]
impl crate::Resettable for AC_DAC_DRC_SFHAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
