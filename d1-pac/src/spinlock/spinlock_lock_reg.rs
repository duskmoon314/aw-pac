#[doc = "Register `SPINLOCK_LOCK_REG%s` reader"]
pub struct R(crate::R<SPINLOCK_LOCK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_LOCK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_LOCK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_LOCK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPINLOCK_LOCK_REG%s` writer"]
pub struct W(crate::W<SPINLOCK_LOCK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPINLOCK_LOCK_REG_SPEC>;
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
impl From<crate::W<SPINLOCK_LOCK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPINLOCK_LOCK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TAKEN` reader - Lock State"]
pub struct TAKEN_R(crate::FieldReader<bool>);
impl TAKEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TAKEN_A::FREE
    }
    #[doc = "Checks if the value of the field is `TAKEN`"]
    #[inline(always)]
    pub fn is_taken(&self) -> bool {
        **self == TAKEN_A::TAKEN
    }
}
impl core::ops::Deref for TAKEN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAKEN` writer - Lock State"]
pub struct TAKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
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
    pub fn taken(&mut self) -> TAKEN_W {
        TAKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spinlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_lock_reg](index.html) module"]
pub struct SPINLOCK_LOCK_REG_SPEC;
impl crate::RegisterSpec for SPINLOCK_LOCK_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_lock_reg::R](R) reader structure"]
impl crate::Readable for SPINLOCK_LOCK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spinlock_lock_reg::W](W) writer structure"]
impl crate::Writable for SPINLOCK_LOCK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPINLOCK_LOCK_REG%s to value 0"]
impl crate::Resettable for SPINLOCK_LOCK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
