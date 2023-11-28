#[doc = "Register `ac_dac_drc_hkl` reader"]
pub type R = crate::R<AC_DAC_DRC_HKL_SPEC>;
#[doc = "Register `ac_dac_drc_hkl` writer"]
pub type W = crate::W<AC_DAC_DRC_HKL_SPEC>;
#[doc = "Field `dac_drc_hkl` reader - The slope of the limiter which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is &lt;50:1>)"]
pub type DAC_DRC_HKL_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_hkl` writer - The slope of the limiter which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is &lt;50:1>)"]
pub type DAC_DRC_HKL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The slope of the limiter which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is &lt;50:1>)"]
    #[inline(always)]
    pub fn dac_drc_hkl(&self) -> DAC_DRC_HKL_R {
        DAC_DRC_HKL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The slope of the limiter which is determined by the equation that Kl = 1/R. R is the ratio of the limiter, which is always an integer. The format is 8.24. (The default value is &lt;50:1>)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_hkl(&mut self) -> DAC_DRC_HKL_W<AC_DAC_DRC_HKL_SPEC> {
        DAC_DRC_HKL_W::new(self, 0)
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
#[doc = "DAC DRC Limiter Slope High Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_hkl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_hkl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_HKL_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_HKL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_hkl::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_HKL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_hkl::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_HKL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_hkl to value 0x05"]
impl crate::Resettable for AC_DAC_DRC_HKL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
