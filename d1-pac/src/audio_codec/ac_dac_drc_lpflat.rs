#[doc = "Register `ac_dac_drc_lpflat` reader"]
pub type R = crate::R<AC_DAC_DRC_LPFLAT_SPEC>;
#[doc = "Register `ac_dac_drc_lpflat` writer"]
pub type W = crate::W<AC_DAC_DRC_LPFLAT_SPEC>;
#[doc = "Field `dac_drc_lpflat` reader - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type DAC_DRC_LPFLAT_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_lpflat` writer - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
pub type DAC_DRC_LPFLAT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    pub fn dac_drc_lpflat(&self) -> DAC_DRC_LPFLAT_R {
        DAC_DRC_LPFLAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The left peak filter attack time parameter setting, which is determined by the equation that AT = 1-exp (-2.2Ts/ta). The format is 3.24. (The default value is 1 ms)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_lpflat(&mut self) -> DAC_DRC_LPFLAT_W<AC_DAC_DRC_LPFLAT_SPEC> {
        DAC_DRC_LPFLAT_W::new(self, 0)
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
#[doc = "DAC DRC Left Peak Filter Low Attack Time Coef Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lpflat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lpflat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_LPFLAT_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LPFLAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_lpflat::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LPFLAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_lpflat::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LPFLAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_lpflat to value 0x77bf"]
impl crate::Resettable for AC_DAC_DRC_LPFLAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x77bf;
}
