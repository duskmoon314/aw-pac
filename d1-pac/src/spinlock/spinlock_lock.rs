#[doc = "Register `spinlock_lock%s` reader"]
pub type R = crate::R<SPINLOCK_LOCK_SPEC>;
#[doc = "Register `spinlock_lock%s` writer"]
pub type W = crate::W<SPINLOCK_LOCK_SPEC>;
#[doc = "Field `taken` reader - Lock State"]
pub type TAKEN_R = crate::BitReader<TAKEN_A>;
#[doc = "Lock State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAKEN_A {
    #[doc = "0: Read 0x0: The lock was previously Not Taken (free). The requester is granted the lock.\n\nWrite 0x0: Set the lock to Not Taken (free)."]
    FREE = 0,
    #[doc = "1: Read 0x1: The lock was previously Taken. The requester is not granted the lock and must retry.\n\nWrite 0x1: No update to the lock value."]
    TAKEN = 1,
}
impl From<TAKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TAKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAKEN_A {
        match self.bits {
            false => TAKEN_A::FREE,
            true => TAKEN_A::TAKEN,
        }
    }
    #[doc = "Read 0x0: The lock was previously Not Taken (free). The requester is granted the lock.\n\nWrite 0x0: Set the lock to Not Taken (free)."]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == TAKEN_A::FREE
    }
    #[doc = "Read 0x1: The lock was previously Taken. The requester is not granted the lock and must retry.\n\nWrite 0x1: No update to the lock value."]
    #[inline(always)]
    pub fn is_taken(&self) -> bool {
        *self == TAKEN_A::TAKEN
    }
}
#[doc = "Field `taken` writer - Lock State"]
pub type TAKEN_W<'a, REG> = crate::BitWriter<'a, REG, TAKEN_A>;
impl<'a, REG> TAKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read 0x0: The lock was previously Not Taken (free). The requester is granted the lock.\n\nWrite 0x0: Set the lock to Not Taken (free)."]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(TAKEN_A::FREE)
    }
    #[doc = "Read 0x1: The lock was previously Taken. The requester is not granted the lock and must retry.\n\nWrite 0x1: No update to the lock value."]
    #[inline(always)]
    pub fn taken(self) -> &'a mut crate::W<REG> {
        self.variant(TAKEN_A::TAKEN)
    }
}
impl R {
    #[doc = "Bit 0 - Lock State"]
    #[inline(always)]
    pub fn taken(&self) -> TAKEN_R {
        TAKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock State"]
    #[inline(always)]
    #[must_use]
    pub fn taken(&mut self) -> TAKEN_W<SPINLOCK_LOCK_SPEC> {
        TAKEN_W::new(self, 0)
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
#[doc = "Spinlock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_LOCK_SPEC;
impl crate::RegisterSpec for SPINLOCK_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock_lock::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spinlock_lock::W`](W) writer structure"]
impl crate::Writable for SPINLOCK_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spinlock_lock%s to value 0"]
impl crate::Resettable for SPINLOCK_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
