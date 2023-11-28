#[doc = "Register `dbg_dll` reader"]
pub type R = crate::R<DBG_DLL_SPEC>;
#[doc = "Field `dbg_dll` reader - "]
pub type DBG_DLL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbg_dll(&self) -> DBG_DLL_R {
        DBG_DLL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Debug DLL Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_dll::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_DLL_SPEC;
impl crate::RegisterSpec for DBG_DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_dll::R`](R) reader structure"]
impl crate::Readable for DBG_DLL_SPEC {}
#[doc = "`reset()` method sets dbg_dll to value 0"]
impl crate::Resettable for DBG_DLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
