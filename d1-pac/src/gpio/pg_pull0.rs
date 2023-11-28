#[doc = "Register `pg_pull0` reader"]
pub type R = crate::R<PG_PULL0_SPEC>;
#[doc = "Register `pg_pull0` writer"]
pub type W = crate::W<PG_PULL0_SPEC>;
#[doc = "Field `pg_pull[0-15]` reader - PG Pull_up/down Select"]
pub type PG_PULL_R = crate::FieldReader<PG_PULL_A>;
#[doc = "PG Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PG_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PG_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PG_PULL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PG_PULL_A {
    type Ux = u8;
}
impl PG_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PG_PULL_A {
        match self.bits {
            0 => PG_PULL_A::PULL_DISABLE,
            1 => PG_PULL_A::PULL_UP,
            2 => PG_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PG_PULL_A::PULL_DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PG_PULL_A::PULL_UP
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PG_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pg_pull[0-15]` writer - PG Pull_up/down Select"]
pub type PG_PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PG_PULL_A>;
impl<'a, REG> PG_PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PG_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PG_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PG_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PG Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pg0_pull` field"]
    #[inline(always)]
    pub fn pg_pull(&self, n: u8) -> PG_PULL_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PG_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg0_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg1_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg2_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg3_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg4_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg5_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg6_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg7_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg8_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg9_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg10_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg11_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg12_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg13_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg14_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg15_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "PG Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pg0_pull` field"]
    #[inline(always)]
    #[must_use]
    pub fn pg_pull(&mut self, n: u8) -> PG_PULL_W<PG_PULL0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        PG_PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg0_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg1_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg2_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg3_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg4_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg5_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg6_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg7_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg8_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg9_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg10_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg11_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg12_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg13_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg14_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - PG Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pg15_pull(&mut self) -> PG_PULL_W<PG_PULL0_SPEC> {
        PG_PULL_W::new(self, 30)
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
#[doc = "PG Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pg_pull0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pg_pull0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PG_PULL0_SPEC;
impl crate::RegisterSpec for PG_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_pull0::R`](R) reader structure"]
impl crate::Readable for PG_PULL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pg_pull0::W`](W) writer structure"]
impl crate::Writable for PG_PULL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pg_pull0 to value 0"]
impl crate::Resettable for PG_PULL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
