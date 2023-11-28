#[doc = "Register `tve_dac_cfg2` reader"]
pub type R = crate::R<TVE_DAC_CFG2_SPEC>;
#[doc = "Register `tve_dac_cfg2` writer"]
pub type W = crate::W<TVE_DAC_CFG2_SPEC>;
#[doc = "Field `r_set` reader - "]
pub type R_SET_R = crate::FieldReader;
#[doc = "Field `r_set` writer - "]
pub type R_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `s2s1` reader - "]
pub type S2S1_R = crate::FieldReader;
#[doc = "Field `s2s1` writer - "]
pub type S2S1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ab` reader - (I config output current for different peak voltage)"]
pub type AB_R = crate::FieldReader;
#[doc = "Field `ab` writer - (I config output current for different peak voltage)"]
pub type AB_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn r_set(&self) -> R_SET_R {
        R_SET_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn s2s1(&self) -> S2S1_R {
        S2S1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - (I config output current for different peak voltage)"]
    #[inline(always)]
    pub fn ab(&self) -> AB_R {
        AB_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn r_set(&mut self) -> R_SET_W<TVE_DAC_CFG2_SPEC> {
        R_SET_W::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn s2s1(&mut self) -> S2S1_W<TVE_DAC_CFG2_SPEC> {
        S2S1_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - (I config output current for different peak voltage)"]
    #[inline(always)]
    #[must_use]
    pub fn ab(&mut self) -> AB_W<TVE_DAC_CFG2_SPEC> {
        AB_W::new(self, 8)
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
#[doc = "TV Encoder DAC CFG2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tve_dac_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tve_dac_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TVE_DAC_CFG2_SPEC;
impl crate::RegisterSpec for TVE_DAC_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_dac_cfg2::R`](R) reader structure"]
impl crate::Readable for TVE_DAC_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tve_dac_cfg2::W`](W) writer structure"]
impl crate::Writable for TVE_DAC_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_dac_cfg2 to value 0"]
impl crate::Resettable for TVE_DAC_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
