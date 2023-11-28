#[doc = "Register `tv_fill_begin%s` reader"]
pub type R = crate::R<TV_FILL_BEGIN_SPEC>;
#[doc = "Register `tv_fill_begin%s` writer"]
pub type W = crate::W<TV_FILL_BEGIN_SPEC>;
#[doc = "Field `fill_begin` reader - Fill Begin"]
pub type FILL_BEGIN_R = crate::FieldReader<u32>;
#[doc = "Field `fill_begin` writer - Fill Begin"]
pub type FILL_BEGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Fill Begin"]
    #[inline(always)]
    pub fn fill_begin(&self) -> FILL_BEGIN_R {
        FILL_BEGIN_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Fill Begin"]
    #[inline(always)]
    #[must_use]
    pub fn fill_begin(&mut self) -> FILL_BEGIN_W<TV_FILL_BEGIN_SPEC> {
        FILL_BEGIN_W::new(self, 0)
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
#[doc = "TV Fill Data Begin Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tv_fill_begin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tv_fill_begin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TV_FILL_BEGIN_SPEC;
impl crate::RegisterSpec for TV_FILL_BEGIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tv_fill_begin::R`](R) reader structure"]
impl crate::Readable for TV_FILL_BEGIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tv_fill_begin::W`](W) writer structure"]
impl crate::Writable for TV_FILL_BEGIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_fill_begin%s to value 0"]
impl crate::Resettable for TV_FILL_BEGIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
