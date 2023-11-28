#[doc = "Register `ths_per` reader"]
pub type R = crate::R<THS_PER_SPEC>;
#[doc = "Register `ths_per` writer"]
pub type W = crate::W<THS_PER_SPEC>;
#[doc = "Field `thermal_per` reader - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
pub type THERMAL_PER_R = crate::FieldReader<u32>;
#[doc = "Field `thermal_per` writer - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
pub type THERMAL_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
    #[inline(always)]
    pub fn thermal_per(&self) -> THERMAL_PER_R {
        THERMAL_PER_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Temperature measurement period\n\n4096*(n + 1)/CLK_IN\n\nThe default value is 10 ms."]
    #[inline(always)]
    #[must_use]
    pub fn thermal_per(&mut self) -> THERMAL_PER_W<THS_PER_SPEC> {
        THERMAL_PER_W::new(self, 12)
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
#[doc = "THS Period Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_per::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_per::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_PER_SPEC;
impl crate::RegisterSpec for THS_PER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_per::R`](R) reader structure"]
impl crate::Readable for THS_PER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_per::W`](W) writer structure"]
impl crate::Writable for THS_PER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_per to value 0x0003_a000"]
impl crate::Resettable for THS_PER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_a000;
}
