#[doc = "Register `spinlock_irq_en` reader"]
pub type R = crate::R<SPINLOCK_IRQ_EN_SPEC>;
#[doc = "Register `spinlock_irq_en` writer"]
pub type W = crate::W<SPINLOCK_IRQ_EN_SPEC>;
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
    pub const fn variant(&self) -> LOCK_IRQ_EN_A {
        match self.bits {
            false => LOCK_IRQ_EN_A::DISABLED,
            true => LOCK_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCK_IRQ_EN_A::DISABLED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOCK_IRQ_EN_A::ENABLED
    }
}
#[doc = "Field `lock_irq_en[0-31]` writer - Lock\\[i\\] Interrupt Enable"]
pub type LOCK_IRQ_EN_W<'a, REG> = crate::BitWriter<'a, REG, LOCK_IRQ_EN_A>;
impl<'a, REG> LOCK_IRQ_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_IRQ_EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Lock\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `lock0_irq_en` field"]
    #[inline(always)]
    pub fn lock_irq_en(&self, n: u8) -> LOCK_IRQ_EN_R {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
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
    #[doc = "Lock\\[i\\] Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `lock0_irq_en` field"]
    #[inline(always)]
    #[must_use]
    pub fn lock_irq_en(&mut self, n: u8) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        LOCK_IRQ_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock0_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock1_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock2_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock3_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock4_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock5_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock6_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock7_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock8_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock9_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock10_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock11_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock12_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock13_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock14_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock15_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock16_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock17_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock18_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock19_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock20_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock21_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock22_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock23_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock24_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock25_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock26_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock27_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock28_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock29_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock30_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Lock\\[i\\] Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lock31_irq_en(&mut self) -> LOCK_IRQ_EN_W<SPINLOCK_IRQ_EN_SPEC> {
        LOCK_IRQ_EN_W::new(self, 31)
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
#[doc = "Spinlock Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spinlock_irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spinlock_irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_IRQ_EN_SPEC;
impl crate::RegisterSpec for SPINLOCK_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock_irq_en::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spinlock_irq_en::W`](W) writer structure"]
impl crate::Writable for SPINLOCK_IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spinlock_irq_en to value 0"]
impl crate::Resettable for SPINLOCK_IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
