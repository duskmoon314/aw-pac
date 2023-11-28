#[doc = "Register `csic_bist_end_addr` reader"]
pub type R = crate::R<CSIC_BIST_END_ADDR_SPEC>;
#[doc = "Register `csic_bist_end_addr` writer"]
pub type W = crate::W<CSIC_BIST_END_ADDR_SPEC>;
#[doc = "Field `bist_end_addr` reader - BIST end address, 32-bit aligned"]
pub type BIST_END_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `bist_end_addr` writer - BIST end address, 32-bit aligned"]
pub type BIST_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BIST end address, 32-bit aligned"]
    #[inline(always)]
    pub fn bist_end_addr(&self) -> BIST_END_ADDR_R {
        BIST_END_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST end address, 32-bit aligned"]
    #[inline(always)]
    #[must_use]
    pub fn bist_end_addr(&mut self) -> BIST_END_ADDR_W<CSIC_BIST_END_ADDR_SPEC> {
        BIST_END_ADDR_W::new(self, 0)
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
#[doc = "CSIC BIST End Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_bist_end_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_bist_end_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_BIST_END_ADDR_SPEC;
impl crate::RegisterSpec for CSIC_BIST_END_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_bist_end_addr::R`](R) reader structure"]
impl crate::Readable for CSIC_BIST_END_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_bist_end_addr::W`](W) writer structure"]
impl crate::Writable for CSIC_BIST_END_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_end_addr to value 0"]
impl crate::Resettable for CSIC_BIST_END_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
