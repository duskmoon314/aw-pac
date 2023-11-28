#[doc = "Register `pll_peri_pat1_ctrl` reader"]
pub type R = crate::R<PLL_PERI_PAT1_CTRL_SPEC>;
#[doc = "Register `pll_peri_pat1_ctrl` writer"]
pub type W = crate::W<PLL_PERI_PAT1_CTRL_SPEC>;
#[doc = "Field `frac_in` reader - Fraction In"]
pub type FRAC_IN_R = crate::FieldReader<u32>;
#[doc = "Field `frac_in` writer - Fraction In"]
pub type FRAC_IN_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `frac_en` reader - Fraction Enable"]
pub type FRAC_EN_R = crate::BitReader;
#[doc = "Field `frac_en` writer - Fraction Enable"]
pub type FRAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dither_en` reader - Dither Enable"]
pub type DITHER_EN_R = crate::BitReader;
#[doc = "Field `dither_en` writer - Dither Enable"]
pub type DITHER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - Fraction In"]
    #[inline(always)]
    pub fn frac_in(&self) -> FRAC_IN_R {
        FRAC_IN_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    pub fn frac_en(&self) -> FRAC_EN_R {
        FRAC_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Dither Enable"]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Fraction In"]
    #[inline(always)]
    #[must_use]
    pub fn frac_in(&mut self) -> FRAC_IN_W<PLL_PERI_PAT1_CTRL_SPEC> {
        FRAC_IN_W::new(self, 0)
    }
    #[doc = "Bit 20 - Fraction Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frac_en(&mut self) -> FRAC_EN_W<PLL_PERI_PAT1_CTRL_SPEC> {
        FRAC_EN_W::new(self, 20)
    }
    #[doc = "Bit 24 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dither_en(&mut self) -> DITHER_EN_W<PLL_PERI_PAT1_CTRL_SPEC> {
        DITHER_EN_W::new(self, 24)
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
#[doc = "PLL_PERI Pattern1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_peri_pat1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_peri_pat1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_PERI_PAT1_CTRL_SPEC;
impl crate::RegisterSpec for PLL_PERI_PAT1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_peri_pat1_ctrl::R`](R) reader structure"]
impl crate::Readable for PLL_PERI_PAT1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_peri_pat1_ctrl::W`](W) writer structure"]
impl crate::Writable for PLL_PERI_PAT1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pll_peri_pat1_ctrl to value 0"]
impl crate::Resettable for PLL_PERI_PAT1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
