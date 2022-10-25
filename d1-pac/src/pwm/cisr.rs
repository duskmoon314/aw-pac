#[doc = "Register `cisr` reader"]
pub struct R(crate::R<CISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cisr` writer"]
pub struct W(crate::W<CISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CISR_SPEC>;
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
impl From<crate::W<CISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cris[0-7]` reader - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CRIS_R = crate::BitReader<CRIS_A>;
#[doc = "Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRIS_A {
    #[doc = "0: `0`"]
    NOT_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<CRIS_A> for bool {
    #[inline(always)]
    fn from(variant: CRIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CRIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRIS_A {
        match self.bits {
            false => CRIS_A::NOT_PENDING,
            true => CRIS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == CRIS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CRIS_A::PENDING
    }
}
#[doc = "Field `cris[0-7]` writer - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CRIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CISR_SPEC, CRIS_A, O>;
impl<'a, const O: u8> CRIS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut W {
        self.variant(CRIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(CRIS_A::PENDING)
    }
}
#[doc = "Field `cfis[0-7]` reader - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CFIS_R = crate::BitReader<CFIS_A>;
#[doc = "Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFIS_A {
    #[doc = "0: `0`"]
    NOT_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<CFIS_A> for bool {
    #[inline(always)]
    fn from(variant: CFIS_A) -> Self {
        variant as u8 != 0
    }
}
impl CFIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFIS_A {
        match self.bits {
            false => CFIS_A::NOT_PENDING,
            true => CFIS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == CFIS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CFIS_A::PENDING
    }
}
#[doc = "Field `cfis[0-7]` writer - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CFIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CISR_SPEC, CFIS_A, O>;
impl<'a, const O: u8> CFIS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut W {
        self.variant(CFIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(CFIS_A::PENDING)
    }
}
impl R {
    #[doc = "Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub unsafe fn cris(&self, n: u8) -> CRIS_R {
        CRIS_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris0(&self) -> CRIS_R {
        CRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris1(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris2(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris3(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris4(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris5(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris6(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cris7(&self) -> CRIS_R {
        CRIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub unsafe fn cfis(&self, n: u8) -> CFIS_R {
        CFIS_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis0(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis1(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis2(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis3(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis4(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis5(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis6(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    pub fn cfis7(&self) -> CFIS_R {
        CFIS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cris<const O: u8>(&mut self) -> CRIS_W<O> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 0 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris0(&mut self) -> CRIS_W<0> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 2 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris1(&mut self) -> CRIS_W<2> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 4 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris2(&mut self) -> CRIS_W<4> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 6 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris3(&mut self) -> CRIS_W<6> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 8 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris4(&mut self) -> CRIS_W<8> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 10 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris5(&mut self) -> CRIS_W<10> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 12 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris6(&mut self) -> CRIS_W<12> {
        CRIS_W::new(self)
    }
    #[doc = "Bit 14 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris7(&mut self) -> CRIS_W<14> {
        CRIS_W::new(self)
    }
    #[doc = "Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cfis<const O: u8>(&mut self) -> CFIS_W<O> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 1 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis0(&mut self) -> CFIS_W<1> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 3 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis1(&mut self) -> CFIS_W<3> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 5 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis2(&mut self) -> CFIS_W<5> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 7 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis3(&mut self) -> CFIS_W<7> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 9 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis4(&mut self) -> CFIS_W<9> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 11 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis5(&mut self) -> CFIS_W<11> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 13 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis6(&mut self) -> CFIS_W<13> {
        CFIS_W::new(self)
    }
    #[doc = "Bit 15 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis7(&mut self) -> CFIS_W<15> {
        CFIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture IRQ Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cisr](index.html) module"]
pub struct CISR_SPEC;
impl crate::RegisterSpec for CISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cisr::R](R) reader structure"]
impl crate::Readable for CISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cisr::W](W) writer structure"]
impl crate::Writable for CISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets cisr to value 0"]
impl crate::Resettable for CISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
