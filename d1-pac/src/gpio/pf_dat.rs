#[doc = "Register `pf_dat` reader"]
pub type R = crate::R<PF_DAT_SPEC>;
#[doc = "Register `pf_dat` writer"]
pub type W = crate::W<PF_DAT_SPEC>;
#[doc = "Field `pf_dat` reader - PF Data"]
pub type PF_DAT_R = crate::FieldReader;
#[doc = "Field `pf_dat` writer - PF Data"]
pub type PF_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&self) -> PF_DAT_R {
        PF_DAT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    #[must_use]
    pub fn pf_dat(&mut self) -> PF_DAT_W<PF_DAT_SPEC> {
        PF_DAT_W::new(self, 0)
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
#[doc = "PF Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_DAT_SPEC;
impl crate::RegisterSpec for PF_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_dat::R`](R) reader structure"]
impl crate::Readable for PF_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_dat::W`](W) writer structure"]
impl crate::Writable for PF_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_dat to value 0"]
impl crate::Resettable for PF_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
