#[doc = "Register `pd_eint_ctl` reader"]
pub type R = crate::R<PD_EINT_CTL_SPEC>;
#[doc = "Register `pd_eint_ctl` writer"]
pub type W = crate::W<PD_EINT_CTL_SPEC>;
#[doc = "Field `eint_ctl[0-22]` reader - External INT Enable"]
pub type EINT_CTL_R = crate::BitReader<EINT_CTL_A>;
#[doc = "External INT Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT_CTL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EINT_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl EINT_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EINT_CTL_A {
        match self.bits {
            false => EINT_CTL_A::DISABLE,
            true => EINT_CTL_A::ENABLE,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EINT_CTL_A::DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EINT_CTL_A::ENABLE
    }
}
#[doc = "Field `eint_ctl[0-22]` writer - External INT Enable"]
pub type EINT_CTL_W<'a, REG> = crate::BitWriter<'a, REG, EINT_CTL_A>;
impl<'a, REG> EINT_CTL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CTL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EINT_CTL_A::ENABLE)
    }
}
impl R {
    #[doc = "External INT Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_ctl` field"]
    #[inline(always)]
    pub fn eint_ctl(&self, n: u8) -> EINT_CTL_R {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        EINT_CTL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - External INT Enable"]
    #[inline(always)]
    pub fn eint0_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External INT Enable"]
    #[inline(always)]
    pub fn eint1_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External INT Enable"]
    #[inline(always)]
    pub fn eint2_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External INT Enable"]
    #[inline(always)]
    pub fn eint3_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External INT Enable"]
    #[inline(always)]
    pub fn eint4_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External INT Enable"]
    #[inline(always)]
    pub fn eint5_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External INT Enable"]
    #[inline(always)]
    pub fn eint6_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External INT Enable"]
    #[inline(always)]
    pub fn eint7_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External INT Enable"]
    #[inline(always)]
    pub fn eint8_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External INT Enable"]
    #[inline(always)]
    pub fn eint9_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External INT Enable"]
    #[inline(always)]
    pub fn eint10_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External INT Enable"]
    #[inline(always)]
    pub fn eint11_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External INT Enable"]
    #[inline(always)]
    pub fn eint12_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External INT Enable"]
    #[inline(always)]
    pub fn eint13_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External INT Enable"]
    #[inline(always)]
    pub fn eint14_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External INT Enable"]
    #[inline(always)]
    pub fn eint15_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External INT Enable"]
    #[inline(always)]
    pub fn eint16_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External INT Enable"]
    #[inline(always)]
    pub fn eint17_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External INT Enable"]
    #[inline(always)]
    pub fn eint18_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - External INT Enable"]
    #[inline(always)]
    pub fn eint19_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - External INT Enable"]
    #[inline(always)]
    pub fn eint20_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - External INT Enable"]
    #[inline(always)]
    pub fn eint21_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - External INT Enable"]
    #[inline(always)]
    pub fn eint22_ctl(&self) -> EINT_CTL_R {
        EINT_CTL_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "External INT Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `eint0_ctl` field"]
    #[inline(always)]
    #[must_use]
    pub fn eint_ctl(&mut self, n: u8) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        EINT_CTL_W::new(self, n)
    }
    #[doc = "Bit 0 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint0_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 0)
    }
    #[doc = "Bit 1 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint1_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 1)
    }
    #[doc = "Bit 2 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint2_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 2)
    }
    #[doc = "Bit 3 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint3_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 3)
    }
    #[doc = "Bit 4 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint4_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 4)
    }
    #[doc = "Bit 5 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint5_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 5)
    }
    #[doc = "Bit 6 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint6_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 6)
    }
    #[doc = "Bit 7 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint7_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 7)
    }
    #[doc = "Bit 8 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint8_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 8)
    }
    #[doc = "Bit 9 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint9_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 9)
    }
    #[doc = "Bit 10 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint10_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 10)
    }
    #[doc = "Bit 11 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint11_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 11)
    }
    #[doc = "Bit 12 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint12_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 12)
    }
    #[doc = "Bit 13 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint13_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 13)
    }
    #[doc = "Bit 14 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint14_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 14)
    }
    #[doc = "Bit 15 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint15_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 15)
    }
    #[doc = "Bit 16 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint16_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 16)
    }
    #[doc = "Bit 17 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint17_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 17)
    }
    #[doc = "Bit 18 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint18_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 18)
    }
    #[doc = "Bit 19 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint19_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 19)
    }
    #[doc = "Bit 20 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint20_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 20)
    }
    #[doc = "Bit 21 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint21_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 21)
    }
    #[doc = "Bit 22 - External INT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eint22_ctl(&mut self) -> EINT_CTL_W<PD_EINT_CTL_SPEC> {
        EINT_CTL_W::new(self, 22)
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
#[doc = "PD External Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_eint_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_eint_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_EINT_CTL_SPEC;
impl crate::RegisterSpec for PD_EINT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_eint_ctl::R`](R) reader structure"]
impl crate::Readable for PD_EINT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_eint_ctl::W`](W) writer structure"]
impl crate::Writable for PD_EINT_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_eint_ctl to value 0"]
impl crate::Resettable for PD_EINT_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
