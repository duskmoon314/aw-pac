#[doc = "Register `spinlock_lock%s` reader"]
pub struct R(crate::R<SPINLOCK_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spinlock_lock%s` writer"]
pub struct W(crate::W<SPINLOCK_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPINLOCK_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPINLOCK_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPINLOCK_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> TAKEN_A {
        match self.bits {
            false => TAKEN_A::FREE,
            true => TAKEN_A::TAKEN,
        }
    }
    #[doc = "Checks if the value of the field is `FREE`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == TAKEN_A::FREE
    }
    #[doc = "Checks if the value of the field is `TAKEN`"]
    #[inline(always)]
    pub fn is_taken(&self) -> bool {
        *self == TAKEN_A::TAKEN
    }
}
#[doc = "Field `taken` writer - Lock State"]
pub type TAKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPINLOCK_LOCK_SPEC, TAKEN_A, O>;
impl<'a, const O: u8> TAKEN_W<'a, O> {
    #[doc = "Read 0x0: The lock was previously Not Taken (free). The requester is granted the lock.\n\nWrite 0x0: Set the lock to Not Taken (free)."]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(TAKEN_A::FREE)
    }
    #[doc = "Read 0x1: The lock was previously Taken. The requester is not granted the lock and must retry.\n\nWrite 0x1: No update to the lock value."]
    #[inline(always)]
    pub fn taken(self) -> &'a mut W {
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
    pub fn taken(&mut self) -> TAKEN_W<0> {
        TAKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spinlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_lock](index.html) module"]
pub struct SPINLOCK_LOCK_SPEC;
impl crate::RegisterSpec for SPINLOCK_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_lock::R](R) reader structure"]
impl crate::Readable for SPINLOCK_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spinlock_lock::W](W) writer structure"]
impl crate::Writable for SPINLOCK_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spinlock_lock%s to value 0"]
impl crate::Resettable for SPINLOCK_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
