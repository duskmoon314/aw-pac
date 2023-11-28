#[doc = "Register `dbg_dlh` reader"]
pub type R = crate::R<DBG_DLH_SPEC>;
#[doc = "Field `dbg_dlh` reader - "]
pub type DBG_DLH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbg_dlh(&self) -> DBG_DLH_R {
        DBG_DLH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Debug DLH Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_dlh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_DLH_SPEC;
impl crate::RegisterSpec for DBG_DLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_dlh::R`](R) reader structure"]
impl crate::Readable for DBG_DLH_SPEC {}
#[doc = "`reset()` method sets dbg_dlh to value 0"]
impl crate::Resettable for DBG_DLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
