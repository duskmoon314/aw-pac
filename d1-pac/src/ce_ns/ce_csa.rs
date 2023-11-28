#[doc = "Register `ce_csa` reader"]
pub type R = crate::R<CE_CSA_SPEC>;
#[doc = "Register `ce_csa` writer"]
pub type W = crate::W<CE_CSA_SPEC>;
#[doc = "Field `cur_src_addr` reader - Current source address"]
pub type CUR_SRC_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current source address"]
    #[inline(always)]
    pub fn cur_src_addr(&self) -> CUR_SRC_ADDR_R {
        CUR_SRC_ADDR_R::new(self.bits)
    }
}
impl W {
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
#[doc = "Current Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_csa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_csa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_CSA_SPEC;
impl crate::RegisterSpec for CE_CSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_csa::R`](R) reader structure"]
impl crate::Readable for CE_CSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_csa::W`](W) writer structure"]
impl crate::Writable for CE_CSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_csa to value 0"]
impl crate::Resettable for CE_CSA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
