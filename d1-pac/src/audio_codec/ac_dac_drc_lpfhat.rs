#[doc = "Register `ac_dac_drc_lpfhat` reader"]
pub type R = crate::R<AC_DAC_DRC_LPFHAT_SPEC>;
#[doc = "Register `ac_dac_drc_lpfhat` writer"]
pub type W = crate::W<AC_DAC_DRC_LPFHAT_SPEC>;
#[doc = "Field `dac_drc_lpfhat` reader - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type DAC_DRC_LPFHAT_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_lpfhat` writer - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type DAC_DRC_LPFHAT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    pub fn dac_drc_lpfhat(&self) -> DAC_DRC_LPFHAT_R {
        DAC_DRC_LPFHAT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_lpfhat(&mut self) -> DAC_DRC_LPFHAT_W<AC_DAC_DRC_LPFHAT_SPEC> {
        DAC_DRC_LPFHAT_W::new(self, 0)
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
#[doc = "DAC DRC Left Peak Filter High Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpfhat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpfhat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_LPFHAT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LPFHAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_lpfhat::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LPFHAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_lpfhat::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LPFHAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_lpfhat to value 0x0b"]
impl crate::Resettable for AC_DAC_DRC_LPFHAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
