#[doc = "Register `pier` reader"]
pub type R = crate::R<PIER_SPEC>;
#[doc = "Register `pier` writer"]
pub type W = crate::W<PIER_SPEC>;
#[doc = "Field `pcie[0-7]` reader - PWM Channel Interrupt Enable"]
pub type PCIE_R = crate::BitReader<PCIE_A>;
#[doc = "PWM Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCIE_A {
    #[doc = "0: PWM Channel Interrupt Disable"]
    DISABLE = 0,
    #[doc = "1: PWM Channel Interrupt Enable"]
    ENABLE = 1,
}
impl From<PCIE_A> for bool {
    #[inline(always)]
    fn from(variant: PCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCIE_A {
        match self.bits {
            false => PCIE_A::DISABLE,
            true => PCIE_A::ENABLE,
        }
    }
    #[doc = "PWM Channel Interrupt Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PCIE_A::DISABLE
    }
    #[doc = "PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PCIE_A::ENABLE
    }
}
#[doc = "Field `pcie[0-7]` writer - PWM Channel Interrupt Enable"]
pub type PCIE_W<'a, REG> = crate::BitWriter<'a, REG, PCIE_A>;
impl<'a, REG> PCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM Channel Interrupt Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PCIE_A::DISABLE)
    }
    #[doc = "PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PCIE_A::ENABLE)
    }
}
#[doc = "Field `pgie[0-3]` reader - PWM Group Interrupt Enable"]
pub type PGIE_R = crate::BitReader<PGIE_A>;
#[doc = "PWM Group Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGIE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<PGIE_A> for bool {
    #[inline(always)]
    fn from(variant: PGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PGIE_A {
        match self.bits {
            false => PGIE_A::DISABLE,
            true => PGIE_A::ENABLE,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PGIE_A::DISABLE
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PGIE_A::ENABLE
    }
}
#[doc = "Field `pgie[0-3]` writer - PWM Group Interrupt Enable"]
pub type PGIE_W<'a, REG> = crate::BitWriter<'a, REG, PGIE_A>;
impl<'a, REG> PGIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PGIE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PGIE_A::ENABLE)
    }
}
impl R {
    #[doc = "PWM Channel Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pcie0` field"]
    #[inline(always)]
    pub fn pcie(&self, n: u8) -> PCIE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PCIE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie0(&self) -> PCIE_R {
        PCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie1(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie2(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie3(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie4(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie5(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie6(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    pub fn pcie7(&self) -> PCIE_R {
        PCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "PWM Group Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pgie0` field"]
    #[inline(always)]
    pub fn pgie(&self, n: u8) -> PGIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PGIE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM Group Interrupt Enable"]
    #[inline(always)]
    pub fn pgie0(&self) -> PGIE_R {
        PGIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM Group Interrupt Enable"]
    #[inline(always)]
    pub fn pgie1(&self) -> PGIE_R {
        PGIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM Group Interrupt Enable"]
    #[inline(always)]
    pub fn pgie2(&self) -> PGIE_R {
        PGIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM Group Interrupt Enable"]
    #[inline(always)]
    pub fn pgie3(&self) -> PGIE_R {
        PGIE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "PWM Channel Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pcie0` field"]
    #[inline(always)]
    #[must_use]
    pub fn pcie(&mut self, n: u8) -> PCIE_W<PIER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PCIE_W::new(self, n)
    }
    #[doc = "Bit 0 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie1(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie2(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie3(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie4(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie5(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie6(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - PWM Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcie7(&mut self) -> PCIE_W<PIER_SPEC> {
        PCIE_W::new(self, 7)
    }
    #[doc = "PWM Group Interrupt Enable\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pgie0` field"]
    #[inline(always)]
    #[must_use]
    pub fn pgie(&mut self, n: u8) -> PGIE_W<PIER_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PGIE_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - PWM Group Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgie0(&mut self) -> PGIE_W<PIER_SPEC> {
        PGIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - PWM Group Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgie1(&mut self) -> PGIE_W<PIER_SPEC> {
        PGIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - PWM Group Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgie2(&mut self) -> PGIE_W<PIER_SPEC> {
        PGIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - PWM Group Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pgie3(&mut self) -> PGIE_W<PIER_SPEC> {
        PGIE_W::new(self, 19)
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
#[doc = "PWM IRQ Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIER_SPEC;
impl crate::RegisterSpec for PIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pier::R`](R) reader structure"]
impl crate::Readable for PIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pier::W`](W) writer structure"]
impl crate::Writable for PIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pier to value 0"]
impl crate::Resettable for PIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
