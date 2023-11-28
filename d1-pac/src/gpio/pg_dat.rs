#[doc = "Register `pg_dat` reader"]
pub type R = crate::R<PG_DAT_SPEC>;
#[doc = "Register `pg_dat` writer"]
pub type W = crate::W<PG_DAT_SPEC>;
#[doc = "Field `pg_dat` reader - PG Data"]
pub type PG_DAT_R = crate::FieldReader<u32>;
#[doc = "Field `pg_dat` writer - PG Data"]
pub type PG_DAT_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - PG Data"]
    #[inline(always)]
    pub fn pg_dat(&self) -> PG_DAT_R {
        PG_DAT_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - PG Data"]
    #[inline(always)]
    #[must_use]
    pub fn pg_dat(&mut self) -> PG_DAT_W<PG_DAT_SPEC> {
        PG_DAT_W::new(self, 0)
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
#[doc = "PG Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_dat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_dat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PG_DAT_SPEC;
impl crate::RegisterSpec for PG_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_dat::R`](R) reader structure"]
impl crate::Readable for PG_DAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pg_dat::W`](W) writer structure"]
impl crate::Writable for PG_DAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pg_dat to value 0"]
impl crate::Resettable for PG_DAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
