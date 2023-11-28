#[doc = "Register `pisr` reader"]
pub type R = crate::R<PISR_SPEC>;
#[doc = "Register `pisr` writer"]
pub type W = crate::W<PISR_SPEC>;
#[doc = "Field `pis[0-7]` reader - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
pub type PIS_R = crate::BitReader<PIS_A>;
#[doc = "PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIS_A {
    #[doc = "0: `0`"]
    NOT_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<PIS_A> for bool {
    #[inline(always)]
    fn from(variant: PIS_A) -> Self {
        variant as u8 != 0
    }
}
impl PIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PIS_A {
        match self.bits {
            false => PIS_A::NOT_PENDING,
            true => PIS_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIS_A::NOT_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIS_A::PENDING
    }
}
#[doc = "Field `pis[0-7]` writer - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
pub type PIS_W<'a, REG> = crate::BitWriter1C<'a, REG, PIS_A>;
impl<'a, REG> PIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(PIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(PIS_A::PENDING)
    }
}
#[doc = "Field `pgis[0-3]` reader - PWM Group Interrupt Status"]
pub type PGIS_R = crate::BitReader;
#[doc = "Field `pgis[0-3]` writer - PWM Group Interrupt Status"]
pub type PGIS_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pis0` field"]
    #[inline(always)]
    pub fn pis(&self, n: u8) -> PIS_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis0(&self) -> PIS_R {
        PIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis1(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis2(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis3(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis4(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis5(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis6(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub fn pis7(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "PWM Group Interrupt Status\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pgis0` field"]
    #[inline(always)]
    pub fn pgis(&self, n: u8) -> PGIS_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PGIS_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM Group Interrupt Status"]
    #[inline(always)]
    pub fn pgis0(&self) -> PGIS_R {
        PGIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM Group Interrupt Status"]
    #[inline(always)]
    pub fn pgis1(&self) -> PGIS_R {
        PGIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM Group Interrupt Status"]
    #[inline(always)]
    pub fn pgis2(&self) -> PGIS_R {
        PGIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM Group Interrupt Status"]
    #[inline(always)]
    pub fn pgis3(&self) -> PGIS_R {
        PGIS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pis0` field"]
    #[inline(always)]
    #[must_use]
    pub fn pis(&mut self, n: u8) -> PIS_W<PISR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PIS_W::new(self, n)
    }
    #[doc = "Bit 0 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis0(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis1(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis2(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis3(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis4(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis5(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis6(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis7(&mut self) -> PIS_W<PISR_SPEC> {
        PIS_W::new(self, 7)
    }
    #[doc = "PWM Group Interrupt Status\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pgis0` field"]
    #[inline(always)]
    #[must_use]
    pub fn pgis(&mut self, n: u8) -> PGIS_W<PISR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PGIS_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis0(&mut self) -> PGIS_W<PISR_SPEC> {
        PGIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis1(&mut self) -> PGIS_W<PISR_SPEC> {
        PGIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis2(&mut self) -> PGIS_W<PISR_SPEC> {
        PGIS_W::new(self, 18)
    }
    #[doc = "Bit 19 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis3(&mut self) -> PGIS_W<PISR_SPEC> {
        PGIS_W::new(self, 19)
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
#[doc = "PWM IRQ Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PISR_SPEC;
impl crate::RegisterSpec for PISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pisr::R`](R) reader structure"]
impl crate::Readable for PISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pisr::W`](W) writer structure"]
impl crate::Writable for PISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_0001;
}
#[doc = "`reset()` method sets pisr to value 0"]
impl crate::Resettable for PISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
