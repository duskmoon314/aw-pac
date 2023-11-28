#[doc = "Register `fboot_info%s` reader"]
pub type R = crate::R<FBOOT_INFO_SPEC>;
#[doc = "Register `fboot_info%s` writer"]
pub type W = crate::W<FBOOT_INFO_SPEC>;
#[doc = "Field `fboot_info` reader - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_R = crate::FieldReader<u32>;
#[doc = "Field `fboot_info` writer - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    pub fn fboot_info(&self) -> FBOOT_INFO_R {
        FBOOT_INFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    #[must_use]
    pub fn fboot_info(&mut self) -> FBOOT_INFO_W<FBOOT_INFO_SPEC> {
        FBOOT_INFO_W::new(self, 0)
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
#[doc = "Fast Boot Information Register \\[01\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fboot_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fboot_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBOOT_INFO_SPEC;
impl crate::RegisterSpec for FBOOT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fboot_info::R`](R) reader structure"]
impl crate::Readable for FBOOT_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fboot_info::W`](W) writer structure"]
impl crate::Writable for FBOOT_INFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fboot_info%s to value 0"]
impl crate::Resettable for FBOOT_INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
