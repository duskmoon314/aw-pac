#[doc = "Register `csic_mbus_req_max` reader"]
pub type R = crate::R<CSIC_MBUS_REQ_MAX_SPEC>;
#[doc = "Register `csic_mbus_req_max` writer"]
pub type W = crate::W<CSIC_MBUS_REQ_MAX_SPEC>;
#[doc = "Field `mcsi_mem_req_max` reader - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
pub type MCSI_MEM_REQ_MAX_R = crate::FieldReader;
#[doc = "Field `mcsi_mem_req_max` writer - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
pub type MCSI_MEM_REQ_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
    #[inline(always)]
    pub fn mcsi_mem_req_max(&self) -> MCSI_MEM_REQ_MAX_R {
        MCSI_MEM_REQ_MAX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum of request commands for the master granted in MCSI_MEM arbiter is N+1."]
    #[inline(always)]
    #[must_use]
    pub fn mcsi_mem_req_max(&mut self) -> MCSI_MEM_REQ_MAX_W<CSIC_MBUS_REQ_MAX_SPEC> {
        MCSI_MEM_REQ_MAX_W::new(self, 0)
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
#[doc = "CSIC MBUS REQ MAX Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_mbus_req_max::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_mbus_req_max::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_MBUS_REQ_MAX_SPEC;
impl crate::RegisterSpec for CSIC_MBUS_REQ_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_mbus_req_max::R`](R) reader structure"]
impl crate::Readable for CSIC_MBUS_REQ_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_mbus_req_max::W`](W) writer structure"]
impl crate::Writable for CSIC_MBUS_REQ_MAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_mbus_req_max to value 0x0f"]
impl crate::Resettable for CSIC_MBUS_REQ_MAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
