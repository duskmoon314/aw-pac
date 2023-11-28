#[doc = "Register `csic_ptn_len` reader"]
pub type R = crate::R<CSIC_PTN_LEN_SPEC>;
#[doc = "Register `csic_ptn_len` writer"]
pub type W = crate::W<CSIC_PTN_LEN_SPEC>;
#[doc = "Field `ptn_len` reader - The pattern length in byte when generating pattern."]
pub type PTN_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `ptn_len` writer - The pattern length in byte when generating pattern."]
pub type PTN_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The pattern length in byte when generating pattern."]
    #[inline(always)]
    pub fn ptn_len(&self) -> PTN_LEN_R {
        PTN_LEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The pattern length in byte when generating pattern."]
    #[inline(always)]
    #[must_use]
    pub fn ptn_len(&mut self) -> PTN_LEN_W<CSIC_PTN_LEN_SPEC> {
        PTN_LEN_W::new(self, 0)
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
#[doc = "CSIC Pattern Generation Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_ptn_len::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_ptn_len::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PTN_LEN_SPEC;
impl crate::RegisterSpec for CSIC_PTN_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_ptn_len::R`](R) reader structure"]
impl crate::Readable for CSIC_PTN_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_ptn_len::W`](W) writer structure"]
impl crate::Writable for CSIC_PTN_LEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_ptn_len to value 0"]
impl crate::Resettable for CSIC_PTN_LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
