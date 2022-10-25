#[doc = "Register `dmac_para%s` reader"]
pub struct R(crate::R<DMAC_PARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_PARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_PARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_PARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `wait_cyc` reader - Wait Clock Cycle"]
pub type WAIT_CYC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Wait Clock Cycle"]
    #[inline(always)]
    pub fn wait_cyc(&self) -> WAIT_CYC_R {
        WAIT_CYC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMAC Channel Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_para](index.html) module"]
pub struct DMAC_PARA_SPEC;
impl crate::RegisterSpec for DMAC_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_para::R](R) reader structure"]
impl crate::Readable for DMAC_PARA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dmac_para%s to value 0"]
impl crate::Resettable for DMAC_PARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
