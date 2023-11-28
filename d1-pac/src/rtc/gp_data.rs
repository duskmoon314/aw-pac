#[doc = "Register `gp_data%s` reader"]
pub type R = crate::R<GP_DATA_SPEC>;
#[doc = "Register `gp_data%s` writer"]
pub type W = crate::W<GP_DATA_SPEC>;
#[doc = "Field `gp_data` reader - "]
pub type GP_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `gp_data` writer - "]
pub type GP_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gp_data(&self) -> GP_DATA_R {
        GP_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gp_data(&mut self) -> GP_DATA_W<GP_DATA_SPEC> {
        GP_DATA_W::new(self, 0)
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
#[doc = "General Purpose Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_DATA_SPEC;
impl crate::RegisterSpec for GP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_data::R`](R) reader structure"]
impl crate::Readable for GP_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_data::W`](W) writer structure"]
impl crate::Writable for GP_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_data%s to value 0"]
impl crate::Resettable for GP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
