#[doc = "Register `pll_video0_bias` reader"]
pub type R = crate::R<PLL_VIDEO0_BIAS_SPEC>;
#[doc = "Register `pll_video0_bias` writer"]
pub type W = crate::W<PLL_VIDEO0_BIAS_SPEC>;
#[doc = "Field `pll_cp` reader - PLL current bias control"]
pub type PLL_CP_R = crate::FieldReader;
#[doc = "Field `pll_cp` writer - PLL current bias control"]
pub type PLL_CP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    pub fn pll_cp(&self) -> PLL_CP_R {
        PLL_CP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - PLL current bias control"]
    #[inline(always)]
    #[must_use]
    pub fn pll_cp(&mut self) -> PLL_CP_W<PLL_VIDEO0_BIAS_SPEC> {
        PLL_CP_W::new(self, 16)
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
#[doc = "PLL_VIDEO0 Bias Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_video0_bias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_video0_bias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_VIDEO0_BIAS_SPEC;
impl crate::RegisterSpec for PLL_VIDEO0_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_video0_bias::R`](R) reader structure"]
impl crate::Readable for PLL_VIDEO0_BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_video0_bias::W`](W) writer structure"]
impl crate::Writable for PLL_VIDEO0_BIAS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_video0_bias to value 0"]
impl crate::Resettable for PLL_VIDEO0_BIAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
