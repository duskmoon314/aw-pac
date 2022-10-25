#[doc = "Register `dbg_dlh` reader"]
pub struct R(crate::R<DBG_DLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_DLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_DLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_DLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dbg_dlh` reader - "]
pub type DBG_DLH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dbg_dlh(&self) -> DBG_DLH_R {
        DBG_DLH_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Debug DLH Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_dlh](index.html) module"]
pub struct DBG_DLH_SPEC;
impl crate::RegisterSpec for DBG_DLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_dlh::R](R) reader structure"]
impl crate::Readable for DBG_DLH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets dbg_dlh to value 0"]
impl crate::Resettable for DBG_DLH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
