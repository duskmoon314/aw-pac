#[doc = "Register `res240_ctrl` reader"]
pub type R = crate::R<RES240_CTRL_SPEC>;
#[doc = "Register `res240_ctrl` writer"]
pub type W = crate::W<RES240_CTRL_SPEC>;
#[doc = "Field `ddr_res240_trim` reader - 240ohms Resistor trimming bit"]
pub type DDR_RES240_TRIM_R = crate::FieldReader;
#[doc = "Field `ddr_res240_trim` writer - 240ohms Resistor trimming bit"]
pub type DDR_RES240_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 240ohms Resistor trimming bit"]
    #[inline(always)]
    pub fn ddr_res240_trim(&self) -> DDR_RES240_TRIM_R {
        DDR_RES240_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 240ohms Resistor trimming bit"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_res240_trim(&mut self) -> DDR_RES240_TRIM_W<RES240_CTRL_SPEC> {
        DDR_RES240_TRIM_W::new(self, 0)
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
#[doc = "240ohms Resistor Manual Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`res240_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`res240_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES240_CTRL_SPEC;
impl crate::RegisterSpec for RES240_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res240_ctrl::R`](R) reader structure"]
impl crate::Readable for RES240_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`res240_ctrl::W`](W) writer structure"]
impl crate::Writable for RES240_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets res240_ctrl to value 0"]
impl crate::Resettable for RES240_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
