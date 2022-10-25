#[doc = "Register `spinlock_lockid%s` reader"]
pub struct R(crate::R<SPINLOCK_LOCKID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_LOCKID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_LOCKID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_LOCKID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Spinlock Lockid Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_lockid](index.html) module"]
pub struct SPINLOCK_LOCKID_SPEC;
impl crate::RegisterSpec for SPINLOCK_LOCKID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_lockid::R](R) reader structure"]
impl crate::Readable for SPINLOCK_LOCKID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spinlock_lockid%s to value 0"]
impl crate::Resettable for SPINLOCK_LOCKID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
