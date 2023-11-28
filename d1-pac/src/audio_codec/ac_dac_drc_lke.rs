#[doc = "Register `ac_dac_drc_lke` reader"]
pub type R = crate::R<AC_DAC_DRC_LKE_SPEC>;
#[doc = "Register `ac_dac_drc_lke` writer"]
pub type W = crate::W<AC_DAC_DRC_LKE_SPEC>;
#[doc = "Field `dac_drc_lke` reader - The slope of the expander, which is determined by the equation that Ke = 1/R. R is the ratio of the expander, which is always an integer and the ke must larger than 50. The format is 8.24. (The default value is &lt;1:5>)"]
pub type DAC_DRC_LKE_R = crate::FieldReader<u16>;
#[doc = "Field `dac_drc_lke` writer - The slope of the expander, which is determined by the equation that Ke = 1/R. R is the ratio of the expander, which is always an integer and the ke must larger than 50. The format is 8.24. (The default value is &lt;1:5>)"]
pub type DAC_DRC_LKE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The slope of the expander, which is determined by the equation that Ke = 1/R. R is the ratio of the expander, which is always an integer and the ke must larger than 50. The format is 8.24. (The default value is &lt;1:5>)"]
    #[inline(always)]
    pub fn dac_drc_lke(&self) -> DAC_DRC_LKE_R {
        DAC_DRC_LKE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The slope of the expander, which is determined by the equation that Ke = 1/R. R is the ratio of the expander, which is always an integer and the ke must larger than 50. The format is 8.24. (The default value is &lt;1:5>)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_drc_lke(&mut self) -> DAC_DRC_LKE_W<AC_DAC_DRC_LKE_SPEC> {
        DAC_DRC_LKE_W::new(self, 0)
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
#[doc = "DAC DRC Expander Slope Low Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_dac_drc_lke::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_dac_drc_lke::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC_DAC_DRC_LKE_SPEC;
impl crate::RegisterSpec for AC_DAC_DRC_LKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_dac_drc_lke::R`](R) reader structure"]
impl crate::Readable for AC_DAC_DRC_LKE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac_dac_drc_lke::W`](W) writer structure"]
impl crate::Writable for AC_DAC_DRC_LKE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ac_dac_drc_lke to value 0"]
impl crate::Resettable for AC_DAC_DRC_LKE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
