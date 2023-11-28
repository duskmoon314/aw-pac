#[doc = "Register `ce_cda` reader"]
pub type R = crate::R<CE_CDA_SPEC>;
#[doc = "Register `ce_cda` writer"]
pub type W = crate::W<CE_CDA_SPEC>;
#[doc = "Field `cur_dst_addr` reader - Current destination address"]
pub type CUR_DST_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current destination address"]
    #[inline(always)]
    pub fn cur_dst_addr(&self) -> CUR_DST_ADDR_R {
        CUR_DST_ADDR_R::new(self.bits)
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
#[doc = "Current Destination Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ce_cda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ce_cda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CE_CDA_SPEC;
impl crate::RegisterSpec for CE_CDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ce_cda::R`](R) reader structure"]
impl crate::Readable for CE_CDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ce_cda::W`](W) writer structure"]
impl crate::Writable for CE_CDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ce_cda to value 0"]
impl crate::Resettable for CE_CDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
