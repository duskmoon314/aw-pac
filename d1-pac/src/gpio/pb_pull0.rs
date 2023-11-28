#[doc = "Register `pb_pull0` reader"]
pub type R = crate::R<PB_PULL0_SPEC>;
#[doc = "Register `pb_pull0` writer"]
pub type W = crate::W<PB_PULL0_SPEC>;
#[doc = "Field `pc_pull[0-12]` reader - PC Pull_up/down Select"]
pub type PC_PULL_R = crate::FieldReader<PC_PULL_A>;
#[doc = "PC Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PC_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PC_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PC_PULL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PC_PULL_A {
    type Ux = u8;
}
impl PC_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PC_PULL_A {
        match self.bits {
            0 => PC_PULL_A::PULL_DISABLE,
            1 => PC_PULL_A::PULL_UP,
            2 => PC_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PC_PULL_A::PULL_DISABLE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PC_PULL_A::PULL_UP
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PC_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pc_pull[0-12]` writer - PC Pull_up/down Select"]
pub type PC_PULL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PC_PULL_A>;
impl<'a, REG> PC_PULL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PC_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(PC_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(PC_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PC Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pc0_pull` field"]
    #[inline(always)]
    pub fn pc_pull(&self, n: u8) -> PC_PULL_R {
        #[allow(clippy::no_effect)]
        [(); 13][n as usize];
        PC_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc0_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc1_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc2_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc3_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc4_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc5_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc6_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc7_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc8_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc9_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc10_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc11_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PC Pull_up/down Select"]
    #[inline(always)]
    pub fn pc12_pull(&self) -> PC_PULL_R {
        PC_PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "PC Pull_up/down Select\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `pc0_pull` field"]
    #[inline(always)]
    #[must_use]
    pub fn pc_pull(&mut self, n: u8) -> PC_PULL_W<PB_PULL0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 13][n as usize];
        PC_PULL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc0_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc1_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc2_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc3_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc5_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc6_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc7_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc8_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc9_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc10_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc11_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PC Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pc12_pull(&mut self) -> PC_PULL_W<PB_PULL0_SPEC> {
        PC_PULL_W::new(self, 24)
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
#[doc = "PB Pull Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_pull0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_pull0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_PULL0_SPEC;
impl crate::RegisterSpec for PB_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_pull0::R`](R) reader structure"]
impl crate::Readable for PB_PULL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_pull0::W`](W) writer structure"]
impl crate::Writable for PB_PULL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pb_pull0 to value 0"]
impl crate::Resettable for PB_PULL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
