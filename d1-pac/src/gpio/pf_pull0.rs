#[doc = "Register `pf_pull0` reader"]
pub type R = crate::R<PF_PULL0_SPEC>;
#[doc = "Register `pf_pull0` writer"]
pub type W = crate::W<PF_PULL0_SPEC>;
#[doc = "Field `pf_pull[0-6]` reader - PF Pull_up/down Select"]
pub type PF_PULL_R = crate::FieldReader<PF_PULL_A>;
#[doc = "PF Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PF_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PF_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PF_PULL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PF_PULL_A {
    type Ux = u8;
}
impl PF_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PF_PULL_A {
        match self.bits {
            0 => PF_PULL_A::PULL_DISABLE,
            1 => PF_PULL_A::PULL_UP,
            2 => PF_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PF_PULL_A::PULL_DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PF_PULL_A::PULL_UP
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PF_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pf_pull[0-6]` writer - PF Pull_up/down Select"]
pub type PF_PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PF_PULL_A>;
impl<'a, REG> PF_PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PF_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PF_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PF_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PF Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pf0_pull` field"]
    #[inline(always)]
    pub fn pf_pull(&self, n: u8) -> PF_PULL_R {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        PF_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf0_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf1_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf2_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf3_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf4_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf5_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf6_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "PF Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pf0_pull` field"]
    #[inline(always)]
    #[must_use]
    pub fn pf_pull(&mut self, n: u8) -> PF_PULL_W<PF_PULL0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 7][n as usize];
        PF_PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf0_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf1_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf2_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf3_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf4_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf5_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PF Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pf6_pull(&mut self) -> PF_PULL_W<PF_PULL0_SPEC> {
        PF_PULL_W::new(self, 12)
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
#[doc = "PF Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pf_pull0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pf_pull0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PF_PULL0_SPEC;
impl crate::RegisterSpec for PF_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_pull0::R`](R) reader structure"]
impl crate::Readable for PF_PULL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pf_pull0::W`](W) writer structure"]
impl crate::Writable for PF_PULL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pf_pull0 to value 0"]
impl crate::Resettable for PF_PULL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
