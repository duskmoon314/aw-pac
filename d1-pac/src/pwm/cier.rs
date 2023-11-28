#[doc = "Register `cier` reader"]
pub type R = crate::R<CIER_SPEC>;
#[doc = "Register `cier` writer"]
pub type W = crate::W<CIER_SPEC>;
#[doc = "Field `crie[0-7]` reader - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
pub type CRIE_R = crate::BitReader<CRIE_A>;
#[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRIE_A {
    #[doc = "0: Capture channel rise lock interrupt disable"]
    DISABLE = 0,
    #[doc = "1: Capture channel rise lock interrupt enable"]
    ENABLE = 1,
}
impl From<CRIE_A> for bool {
    #[inline(always)]
    fn from(variant: CRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRIE_A {
        match self.bits {
            false => CRIE_A::DISABLE,
            true => CRIE_A::ENABLE,
        }
    }
    #[doc = "Capture channel rise lock interrupt disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRIE_A::DISABLE
    }
    #[doc = "Capture channel rise lock interrupt enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRIE_A::ENABLE
    }
}
#[doc = "Field `crie[0-7]` writer - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
pub type CRIE_W<'a, REG> = crate::BitWriter<'a, REG, CRIE_A>;
impl<'a, REG> CRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture channel rise lock interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CRIE_A::DISABLE)
    }
    #[doc = "Capture channel rise lock interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CRIE_A::ENABLE)
    }
}
#[doc = "Field `cfie[0-7]` reader - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
pub type CFIE_R = crate::BitReader<CFIE_A>;
#[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFIE_A {
    #[doc = "0: Capture channel fall lock interrupt disable"]
    DISABLE = 0,
    #[doc = "1: Capture channel fall lock interrupt enable"]
    ENABLE = 1,
}
impl From<CFIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFIE_A {
        match self.bits {
            false => CFIE_A::DISABLE,
            true => CFIE_A::ENABLE,
        }
    }
    #[doc = "Capture channel fall lock interrupt disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CFIE_A::DISABLE
    }
    #[doc = "Capture channel fall lock interrupt enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CFIE_A::ENABLE
    }
}
#[doc = "Field `cfie[0-7]` writer - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
pub type CFIE_W<'a, REG> = crate::BitWriter<'a, REG, CFIE_A>;
impl<'a, REG> CFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture channel fall lock interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE_A::DISABLE)
    }
    #[doc = "Capture channel fall lock interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CFIE_A::ENABLE)
    }
}
impl R {
    #[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `crie0` field"]
    #[inline(always)]
    pub fn crie(&self, n: u8) -> CRIE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CRIE_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Bit 0 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie0(&self) -> CRIE_R {
        CRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie1(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie2(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie3(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie4(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie5(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie6(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn crie7(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cfie0` field"]
    #[inline(always)]
    pub fn cfie(&self, n: u8) -> CFIE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CFIE_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie0(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie1(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie2(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie3(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie4(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie5(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie6(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    pub fn cfie7(&self) -> CFIE_R {
        CFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `crie0` field"]
    #[inline(always)]
    #[must_use]
    pub fn crie(&mut self, n: u8) -> CRIE_W<CIER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CRIE_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie0(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 0)
    }
    #[doc = "Bit 2 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie1(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 2)
    }
    #[doc = "Bit 4 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie2(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie3(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie4(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 8)
    }
    #[doc = "Bit 10 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie5(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie6(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 12)
    }
    #[doc = "Bit 14 - If the enable bit is set to 1, when the capture channel captures rising edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn crie7(&mut self) -> CRIE_W<CIER_SPEC> {
        CRIE_W::new(self, 14)
    }
    #[doc = "If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `cfie0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self, n: u8) -> CFIE_W<CIER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CFIE_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie0(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 1)
    }
    #[doc = "Bit 3 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie1(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 3)
    }
    #[doc = "Bit 5 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie2(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 5)
    }
    #[doc = "Bit 7 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie3(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 7)
    }
    #[doc = "Bit 9 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie4(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 9)
    }
    #[doc = "Bit 11 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie5(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 11)
    }
    #[doc = "Bit 13 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie6(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 13)
    }
    #[doc = "Bit 15 - If the enable bit is set to 1, when the capture channel captures falling edge, it generates a capture channel pending."]
    #[inline(always)]
    #[must_use]
    pub fn cfie7(&mut self) -> CFIE_W<CIER_SPEC> {
        CFIE_W::new(self, 15)
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
#[doc = "Capture IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cier to value 0"]
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
