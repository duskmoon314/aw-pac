#[doc = "Register `pb_dat` reader"]
pub type R = crate::R<PB_DAT_SPEC>;
#[doc = "Register `pb_dat` writer"]
pub type W = crate::W<PB_DAT_SPEC>;
#[doc = "Field `pb_dat` reader - "]
pub type PB_DAT_R = crate::FieldReader<u16>;
#[doc = "Field `pb_dat` writer - "]
pub type PB_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn pb_dat(&self) -> PB_DAT_R {
        PB_DAT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn pb_dat(&mut self) -> PB_DAT_W<PB_DAT_SPEC> {
        PB_DAT_W::new(self, 0)
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
#[doc = "PB Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_DAT_SPEC;
impl crate::RegisterSpec for PB_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_dat::R`](R) reader structure"]
impl crate::Readable for PB_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_dat::W`](W) writer structure"]
impl crate::Writable for PB_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_dat to value 0"]
impl crate::Resettable for PB_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
