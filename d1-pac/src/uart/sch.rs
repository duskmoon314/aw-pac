#[doc = "Register `sch` reader"]
pub type R = crate::R<SCH_SPEC>;
#[doc = "Register `sch` writer"]
pub type W = crate::W<SCH_SPEC>;
#[doc = "Field `scratch` reader - "]
pub type SCRATCH_R = crate::FieldReader;
#[doc = "Field `scratch` writer - "]
pub type SCRATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn scratch(&self) -> SCRATCH_R {
        SCRATCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn scratch(&mut self) -> SCRATCH_W<SCH_SPEC> {
        SCRATCH_W::new(self, 0)
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
#[doc = "UART Scratch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCH_SPEC;
impl crate::RegisterSpec for SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sch::R`](R) reader structure"]
impl crate::Readable for SCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sch::W`](W) writer structure"]
impl crate::Writable for SCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sch to value 0"]
impl crate::Resettable for SCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
