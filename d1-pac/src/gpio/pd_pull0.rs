#[doc = "Register `pd_pull0` reader"]
pub type R = crate::R<PD_PULL0_SPEC>;
#[doc = "Register `pd_pull0` writer"]
pub type W = crate::W<PD_PULL0_SPEC>;
#[doc = "Field `pd_pull[0-15]` reader - PD Pull_up/down Select"]
pub type PD_PULL_R = crate::FieldReader<PD_PULL_A>;
#[doc = "PD Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD_PULL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PD_PULL_A {
    type Ux = u8;
}
impl PD_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PD_PULL_A {
        match self.bits {
            0 => PD_PULL_A::PULL_DISABLE,
            1 => PD_PULL_A::PULL_UP,
            2 => PD_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PD_PULL_A::PULL_DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PD_PULL_A::PULL_UP
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PD_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pd_pull[0-15]` writer - PD Pull_up/down Select"]
pub type PD_PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PD_PULL_A>;
impl<'a, REG> PD_PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PD_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PD_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PD_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PD Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pd0_pull` field"]
    #[inline(always)]
    pub fn pd_pull(&self, n: u8) -> PD_PULL_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PD_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd0_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd1_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd2_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd3_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd4_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd5_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd6_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd7_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd8_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd9_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd10_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd11_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd12_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd13_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd14_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd15_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "PD Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pd0_pull` field"]
    #[inline(always)]
    #[must_use]
    pub fn pd_pull(&mut self, n: u8) -> PD_PULL_W<PD_PULL0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PD_PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd0_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd1_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd2_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd3_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd4_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd5_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd6_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd7_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd8_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd9_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd10_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd11_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd12_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd13_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd14_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd15_pull(&mut self) -> PD_PULL_W<PD_PULL0_SPEC> {
        PD_PULL_W::new(self, 30)
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
#[doc = "PD Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_pull0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_pull0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_PULL0_SPEC;
impl crate::RegisterSpec for PD_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_pull0::R`](R) reader structure"]
impl crate::Readable for PD_PULL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_pull0::W`](W) writer structure"]
impl crate::Writable for PD_PULL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_pull0 to value 0"]
impl crate::Resettable for PD_PULL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
