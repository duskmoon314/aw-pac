#[doc = "Register `dmac_para%s` reader"]
pub type R = crate::R<DMAC_PARA_SPEC>;
#[doc = "Field `wait_cyc` reader - Wait Clock Cycle"]
pub type WAIT_CYC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Wait Clock Cycle"]
    #[inline(always)]
    pub fn wait_cyc(&self) -> WAIT_CYC_R {
        WAIT_CYC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAC Channel Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_para::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_PARA_SPEC;
impl crate::RegisterSpec for DMAC_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_para::R`](R) reader structure"]
impl crate::Readable for DMAC_PARA_SPEC {}
#[doc = "`reset()` method sets dmac_para%s to value 0"]
impl crate::Resettable for DMAC_PARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
