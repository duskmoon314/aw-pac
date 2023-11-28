#[doc = "Register `spinlock_lockid%s` reader"]
pub type R = crate::R<SPINLOCK_LOCKID_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPINLOCK_LOCKID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Spinlock Lockid Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_lockid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_LOCKID_SPEC;
impl crate::RegisterSpec for SPINLOCK_LOCKID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock_lockid::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_LOCKID_SPEC {}
#[doc = "`reset()` method sets spinlock_lockid%s to value 0"]
impl crate::Resettable for SPINLOCK_LOCKID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
