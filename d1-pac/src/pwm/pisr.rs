#[doc = "Register `pisr` reader"]
pub struct R(crate::R<PISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pisr` writer"]
pub struct W(crate::W<PISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PISR_SPEC>;
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
impl From<crate::W<PISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PISR_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> PIS_A {
        match self.bits {
            false => PIS_A::NOT_PENDING,
            true => PIS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIS_A::PENDING
    }
}
#[doc = "Field `pis[0-7]` writer - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
pub type PIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PISR_SPEC, PIS_A, O>;
impl<'a, const O: u8> PIS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut W {
        self.variant(PIS_A::NOT_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(PIS_A::PENDING)
    }
}
#[doc = "Field `pgis[0-3]` reader - PWM Group Interrupt Status"]
pub type PGIS_R = crate::BitReader<bool>;
#[doc = "Field `pgis[0-3]` writer - PWM Group Interrupt Status"]
pub type PGIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PISR_SPEC, bool, O>;
impl R {
    #[doc = "PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    pub unsafe fn pis(&self, n: u8) -> PIS_R {
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
    #[doc = "PWM Group Interrupt Status"]
    #[inline(always)]
    pub unsafe fn pgis(&self, n: u8) -> PGIS_R {
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
    #[doc = "PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pis<const O: u8>(&mut self) -> PIS_W<O> {
        PIS_W::new(self)
    }
    #[doc = "Bit 0 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis0(&mut self) -> PIS_W<0> {
        PIS_W::new(self)
    }
    #[doc = "Bit 1 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis1(&mut self) -> PIS_W<1> {
        PIS_W::new(self)
    }
    #[doc = "Bit 2 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis2(&mut self) -> PIS_W<2> {
        PIS_W::new(self)
    }
    #[doc = "Bit 3 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis3(&mut self) -> PIS_W<3> {
        PIS_W::new(self)
    }
    #[doc = "Bit 4 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis4(&mut self) -> PIS_W<4> {
        PIS_W::new(self)
    }
    #[doc = "Bit 5 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis5(&mut self) -> PIS_W<5> {
        PIS_W::new(self)
    }
    #[doc = "Bit 6 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis6(&mut self) -> PIS_W<6> {
        PIS_W::new(self)
    }
    #[doc = "Bit 7 - PWM Channel Interrupt Status\n\nWhen the PWM channel counter reaches the Entire Cycle Value, this bit is set 1 by hardware. Writing 1 to clear this bit.\n\nReads 0: PWM channel 0 interrupt is not pending.\n\nReads 1: PWM channel 0 interrupt is pending.\n\nWrites 0: No effect.\n\nWrites 1: Clear PWM channel 0 interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn pis7(&mut self) -> PIS_W<7> {
        PIS_W::new(self)
    }
    #[doc = "PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pgis<const O: u8>(&mut self) -> PGIS_W<O> {
        PGIS_W::new(self)
    }
    #[doc = "Bit 16 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis0(&mut self) -> PGIS_W<16> {
        PGIS_W::new(self)
    }
    #[doc = "Bit 17 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis1(&mut self) -> PGIS_W<17> {
        PGIS_W::new(self)
    }
    #[doc = "Bit 18 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis2(&mut self) -> PGIS_W<18> {
        PGIS_W::new(self)
    }
    #[doc = "Bit 19 - PWM Group Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn pgis3(&mut self) -> PGIS_W<19> {
        PGIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM IRQ Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pisr](index.html) module"]
pub struct PISR_SPEC;
impl crate::RegisterSpec for PISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pisr::R](R) reader structure"]
impl crate::Readable for PISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pisr::W](W) writer structure"]
impl crate::Writable for PISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_0001;
}
#[doc = "`reset()` method sets pisr to value 0"]
impl crate::Resettable for PISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
