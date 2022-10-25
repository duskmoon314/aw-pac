#[doc = "Register `spinlock_systatus` reader"]
pub struct R(crate::R<SPINLOCK_SYSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_SYSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_SYSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_SYSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `iu0` reader - In-Use flag0"]
pub type IU0_R = crate::BitReader<IU0_A>;
#[doc = "In-Use flag0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IU0_A {
    #[doc = "0: All lock registers 0-31 are not taken"]
    ALL_NOT_TAKEN = 0,
    #[doc = "1: At least one of the lock registers 0-31 is taken"]
    AT_LEAST_ONE_TAKEN = 1,
}
impl From<IU0_A> for bool {
    #[inline(always)]
    fn from(variant: IU0_A) -> Self {
        variant as u8 != 0
    }
}
impl IU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IU0_A {
        match self.bits {
            false => IU0_A::ALL_NOT_TAKEN,
            true => IU0_A::AT_LEAST_ONE_TAKEN,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_NOT_TAKEN`"]
    #[inline(always)]
    pub fn is_all_not_taken(&self) -> bool {
        *self == IU0_A::ALL_NOT_TAKEN
    }
    #[doc = "Checks if the value of the field is `AT_LEAST_ONE_TAKEN`"]
    #[inline(always)]
    pub fn is_at_least_one_taken(&self) -> bool {
        *self == IU0_A::AT_LEAST_ONE_TAKEN
    }
}
#[doc = "Field `locks_num` reader - Number of lock registers implemented"]
pub type LOCKS_NUM_R = crate::FieldReader<u8, LOCKS_NUM_A>;
#[doc = "Number of lock registers implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCKS_NUM_A {
    #[doc = "0: 256 lock registers"]
    N256 = 0,
    #[doc = "1: 32 lock registers"]
    N32 = 1,
    #[doc = "2: 64 lock registers"]
    N64 = 2,
    #[doc = "3: 128 lock registers"]
    N128 = 3,
}
impl From<LOCKS_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCKS_NUM_A) -> Self {
        variant as _
    }
}
impl LOCKS_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKS_NUM_A {
        match self.bits {
            0 => LOCKS_NUM_A::N256,
            1 => LOCKS_NUM_A::N32,
            2 => LOCKS_NUM_A::N64,
            3 => LOCKS_NUM_A::N128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `N256`"]
    #[inline(always)]
    pub fn is_n256(&self) -> bool {
        *self == LOCKS_NUM_A::N256
    }
    #[doc = "Checks if the value of the field is `N32`"]
    #[inline(always)]
    pub fn is_n32(&self) -> bool {
        *self == LOCKS_NUM_A::N32
    }
    #[doc = "Checks if the value of the field is `N64`"]
    #[inline(always)]
    pub fn is_n64(&self) -> bool {
        *self == LOCKS_NUM_A::N64
    }
    #[doc = "Checks if the value of the field is `N128`"]
    #[inline(always)]
    pub fn is_n128(&self) -> bool {
        *self == LOCKS_NUM_A::N128
    }
}
impl R {
    #[doc = "Bit 8 - In-Use flag0"]
    #[inline(always)]
    pub fn iu0(&self) -> IU0_R {
        IU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Number of lock registers implemented"]
    #[inline(always)]
    pub fn locks_num(&self) -> LOCKS_NUM_R {
        LOCKS_NUM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "Spinlock System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_systatus](index.html) module"]
pub struct SPINLOCK_SYSTATUS_SPEC;
impl crate::RegisterSpec for SPINLOCK_SYSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_systatus::R](R) reader structure"]
impl crate::Readable for SPINLOCK_SYSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spinlock_systatus to value 0"]
impl crate::Resettable for SPINLOCK_SYSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
