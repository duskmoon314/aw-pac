#[doc = "Register `intosc_clk_prescal` reader"]
pub type R = crate::R<INTOSC_CLK_PRESCAL_SPEC>;
#[doc = "Register `intosc_clk_prescal` writer"]
pub type W = crate::W<INTOSC_CLK_PRESCAL_SPEC>;
#[doc = "Field `intosc_32k_clk_prescal` reader - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_R = crate::FieldReader;
#[doc = "Field `intosc_32k_clk_prescal` writer - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
pub type INTOSC_32K_CLK_PRESCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
    #[inline(always)]
    pub fn intosc_32k_clk_prescal(&self) -> INTOSC_32K_CLK_PRESCAL_R {
        INTOSC_32K_CLK_PRESCAL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Internal OSC 32K Clock Prescalar value N.\n\nThe clock output = Internal RC/32/N."]
    #[inline(always)]
    #[must_use]
    pub fn intosc_32k_clk_prescal(&mut self) -> INTOSC_32K_CLK_PRESCAL_W<INTOSC_CLK_PRESCAL_SPEC> {
        INTOSC_32K_CLK_PRESCAL_W::new(self, 0)
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
#[doc = "Internal OSC Clock Pre-scalar Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intosc_clk_prescal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intosc_clk_prescal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTOSC_CLK_PRESCAL_SPEC;
impl crate::RegisterSpec for INTOSC_CLK_PRESCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intosc_clk_prescal::R`](R) reader structure"]
impl crate::Readable for INTOSC_CLK_PRESCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intosc_clk_prescal::W`](W) writer structure"]
impl crate::Writable for INTOSC_CLK_PRESCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets intosc_clk_prescal to value 0x0f"]
impl crate::Resettable for INTOSC_CLK_PRESCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
