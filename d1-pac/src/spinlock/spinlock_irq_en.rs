#[doc = "Register `spinlock_irq_en` reader"]
pub struct R(crate::R<SPINLOCK_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPINLOCK_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPINLOCK_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPINLOCK_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spinlock_irq_en` writer"]
pub struct W(crate::W<SPINLOCK_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPINLOCK_IRQ_EN_SPEC>;
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
impl From<crate::W<SPINLOCK_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPINLOCK_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lock_irq_en[0-31]` reader - Lock\\[i\\] Interrupt Enable"]
pub type LOCK_IRQ_EN_R = crate::BitReader<LOCK_IRQ_EN_A>;
#[doc = "Lock\\[i\\] Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<LOCK_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_IRQ_EN_A {
        match self.bits {
            false => LOCK_IRQ_EN_A::DISABLED,
            true => LOCK_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCK_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCK_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `lock_irq_en[0-31]` writer - Lock\\[i\\] Interrupt Enable"]
pub type LOCK_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPINLOCK_IRQ_EN_SPEC, LOCK_IRQ_EN_A, O>;
impl<'a, const O: u8> LOCK_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCK_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOCK_IRQ_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub unsafe fn lock_irq_en(&self, n: u8) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock0_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock1_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock2_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock3_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock4_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock5_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock6_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock7_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock8_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock9_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock10_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock11_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock12_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock13_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock14_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock15_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock16_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock17_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock18_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock19_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock20_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock21_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock22_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock23_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock24_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock25_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock26_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock27_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock28_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock29_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock30_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    pub fn lock31_irq_en(&self) -> LOCK_IRQ_EN_R {
        LOCK_IRQ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn lock_irq_en<const O: u8>(&mut self) -> LOCK_IRQ_EN_W<O> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 0 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock0_irq_en(&mut self) -> LOCK_IRQ_EN_W<0> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock1_irq_en(&mut self) -> LOCK_IRQ_EN_W<1> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock2_irq_en(&mut self) -> LOCK_IRQ_EN_W<2> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 3 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock3_irq_en(&mut self) -> LOCK_IRQ_EN_W<3> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock4_irq_en(&mut self) -> LOCK_IRQ_EN_W<4> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 5 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock5_irq_en(&mut self) -> LOCK_IRQ_EN_W<5> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 6 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock6_irq_en(&mut self) -> LOCK_IRQ_EN_W<6> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 7 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock7_irq_en(&mut self) -> LOCK_IRQ_EN_W<7> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 8 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock8_irq_en(&mut self) -> LOCK_IRQ_EN_W<8> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 9 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock9_irq_en(&mut self) -> LOCK_IRQ_EN_W<9> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 10 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock10_irq_en(&mut self) -> LOCK_IRQ_EN_W<10> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 11 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock11_irq_en(&mut self) -> LOCK_IRQ_EN_W<11> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 12 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock12_irq_en(&mut self) -> LOCK_IRQ_EN_W<12> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 13 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock13_irq_en(&mut self) -> LOCK_IRQ_EN_W<13> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 14 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock14_irq_en(&mut self) -> LOCK_IRQ_EN_W<14> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 15 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock15_irq_en(&mut self) -> LOCK_IRQ_EN_W<15> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 16 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock16_irq_en(&mut self) -> LOCK_IRQ_EN_W<16> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 17 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock17_irq_en(&mut self) -> LOCK_IRQ_EN_W<17> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 18 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock18_irq_en(&mut self) -> LOCK_IRQ_EN_W<18> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 19 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock19_irq_en(&mut self) -> LOCK_IRQ_EN_W<19> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 20 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock20_irq_en(&mut self) -> LOCK_IRQ_EN_W<20> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 21 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock21_irq_en(&mut self) -> LOCK_IRQ_EN_W<21> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 22 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock22_irq_en(&mut self) -> LOCK_IRQ_EN_W<22> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 23 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock23_irq_en(&mut self) -> LOCK_IRQ_EN_W<23> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 24 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock24_irq_en(&mut self) -> LOCK_IRQ_EN_W<24> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 25 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock25_irq_en(&mut self) -> LOCK_IRQ_EN_W<25> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 26 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock26_irq_en(&mut self) -> LOCK_IRQ_EN_W<26> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 27 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock27_irq_en(&mut self) -> LOCK_IRQ_EN_W<27> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 28 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock28_irq_en(&mut self) -> LOCK_IRQ_EN_W<28> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 29 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock29_irq_en(&mut self) -> LOCK_IRQ_EN_W<29> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 30 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock30_irq_en(&mut self) -> LOCK_IRQ_EN_W<30> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 31 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock31_irq_en(&mut self) -> LOCK_IRQ_EN_W<31> {
        LOCK_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spinlock Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spinlock_irq_en](index.html) module"]
pub struct SPINLOCK_IRQ_EN_SPEC;
impl crate::RegisterSpec for SPINLOCK_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spinlock_irq_en::R](R) reader structure"]
impl crate::Readable for SPINLOCK_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spinlock_irq_en::W](W) writer structure"]
impl crate::Writable for SPINLOCK_IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spinlock_irq_en to value 0"]
impl crate::Resettable for SPINLOCK_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
