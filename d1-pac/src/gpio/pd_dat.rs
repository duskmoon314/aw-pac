#[doc = "Register `pd_dat` reader"]
pub type R = crate::R<PD_DAT_SPEC>;
#[doc = "Register `pd_dat` writer"]
pub type W = crate::W<PD_DAT_SPEC>;
#[doc = "Field `pd_dat` reader - "]
pub type PD_DAT_R = crate::FieldReader<u32>;
#[doc = "Field `pd_dat` writer - "]
pub type PD_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn pd_dat(&self) -> PD_DAT_R {
        PD_DAT_R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    #[must_use]
    pub fn pd_dat(&mut self) -> PD_DAT_W<PD_DAT_SPEC> {
        PD_DAT_W::new(self, 0)
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
#[doc = "PD Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_DAT_SPEC;
impl crate::RegisterSpec for PD_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_dat::R`](R) reader structure"]
impl crate::Readable for PD_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_dat::W`](W) writer structure"]
impl crate::Writable for PD_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_dat to value 0"]
impl crate::Resettable for PD_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
