#[doc = "Register `cisr` reader"]
pub type R = crate::R<CISR_SPEC>;
#[doc = "Register `cisr` writer"]
pub type W = crate::W<CISR_SPEC>;
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
    pub const fn variant(&self) -> CRIS_A {
        match self.bits {
            false => CRIS_A::NOT_PENDING,
            true => CRIS_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == CRIS_A::NOT_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CRIS_A::PENDING
    }
}
#[doc = "Field `cris[0-7]` writer - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CRIS_W<'a, REG> = crate::BitWriter1C<'a, REG, CRIS_A>;
impl<'a, REG> CRIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(CRIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
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
    pub const fn variant(&self) -> CFIS_A {
        match self.bits {
            false => CFIS_A::NOT_PENDING,
            true => CFIS_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == CFIS_A::NOT_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == CFIS_A::PENDING
    }
}
#[doc = "Field `cfis[0-7]` writer - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
pub type CFIS_W<'a, REG> = crate::BitWriter1C<'a, REG, CFIS_A>;
impl<'a, REG> CFIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(CFIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(CFIS_A::PENDING)
    }
}
impl R {
    #[doc = "Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cris0` field"]
    #[inline(always)]
    pub fn cris(&self, n: u8) -> CRIS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
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
    #[doc = "Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cfis0` field"]
    #[inline(always)]
    pub fn cfis(&self, n: u8) -> CFIS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
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
    #[doc = "Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cris0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cris(&mut self, n: u8) -> CRIS_W<CISR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CRIS_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris0(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris1(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 2)
    }
    #[doc = "Bit 4 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris2(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris3(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 6)
    }
    #[doc = "Bit 8 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris4(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 8)
    }
    #[doc = "Bit 10 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris5(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 10)
    }
    #[doc = "Bit 12 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris6(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 12)
    }
    #[doc = "Bit 14 - Status of the capture channel rising lock interrupt\n\nWhen the capture channel captures rising edge, if the rise lock interrupt ( CRIE ) is enabled, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cris7(&mut self) -> CRIS_W<CISR_SPEC> {
        CRIS_W::new(self, 14)
    }
    #[doc = "Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cfis0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cfis(&mut self, n: u8) -> CFIS_W<CISR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CFIS_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis0(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis1(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 3)
    }
    #[doc = "Bit 5 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis2(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 5)
    }
    #[doc = "Bit 7 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis3(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 7)
    }
    #[doc = "Bit 9 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis4(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 9)
    }
    #[doc = "Bit 11 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis5(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 11)
    }
    #[doc = "Bit 13 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis6(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 13)
    }
    #[doc = "Bit 15 - Status of the capture channel falling lock interrupt\n\nWhen the capture channel captures falling edge, if the fall lock interrupt ( CFIE ) is enabled, this bit is set to 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: The capture channel interrupt is not pending.\n\nReads 1: The capture channel interrupt is pending.\n\nWrites 0: no effect.\n\nWrites 1: Clear the status of the capture channel interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfis7(&mut self) -> CFIS_W<CISR_SPEC> {
        CFIS_W::new(self, 15)
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
#[doc = "Capture IRQ Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CISR_SPEC;
impl crate::RegisterSpec for CISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cisr::R`](R) reader structure"]
impl crate::Readable for CISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cisr::W`](W) writer structure"]
impl crate::Writable for CISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets cisr to value 0"]
impl crate::Resettable for CISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
