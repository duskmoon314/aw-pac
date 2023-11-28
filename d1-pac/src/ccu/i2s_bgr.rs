#[doc = "Register `i2s_bgr` reader"]
pub type R = crate::R<I2S_BGR_SPEC>;
#[doc = "Register `i2s_bgr` writer"]
pub type W = crate::W<I2S_BGR_SPEC>;
#[doc = "Field `i2s_gating[0-2]` reader - Gating Clock"]
pub type I2S_GATING_R = crate::BitReader<I2S_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<I2S_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2S_GATING_A {
        match self.bits {
            false => I2S_GATING_A::MASK,
            true => I2S_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == I2S_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == I2S_GATING_A::PASS
    }
}
#[doc = "Field `i2s_gating[0-2]` writer - Gating Clock"]
pub type I2S_GATING_W<'a, REG> = crate::BitWriter<'a, REG, I2S_GATING_A>;
impl<'a, REG> I2S_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_GATING_A::PASS)
    }
}
#[doc = "Field `i2s_rst[0-2]` reader - Reset"]
pub type I2S_RST_R = crate::BitReader<I2S_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<I2S_RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2S_RST_A {
        match self.bits {
            false => I2S_RST_A::ASSERT,
            true => I2S_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == I2S_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == I2S_RST_A::DEASSERT
    }
}
#[doc = "Field `i2s_rst[0-2]` writer - Reset"]
pub type I2S_RST_W<'a, REG> = crate::BitWriter<'a, REG, I2S_RST_A>;
impl<'a, REG> I2S_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(I2S_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `i2s0_gating` field"]
    #[inline(always)]
    pub fn i2s_gating(&self, n: u8) -> I2S_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        I2S_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn i2s0_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn i2s1_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn i2s2_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `i2s0_rst` field"]
    #[inline(always)]
    pub fn i2s_rst(&self, n: u8) -> I2S_RST_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        I2S_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn i2s2_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `i2s0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_gating(&mut self, n: u8) -> I2S_GATING_W<I2S_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        I2S_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_gating(&mut self) -> I2S_GATING_W<I2S_BGR_SPEC> {
        I2S_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_gating(&mut self) -> I2S_GATING_W<I2S_BGR_SPEC> {
        I2S_GATING_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_gating(&mut self) -> I2S_GATING_W<I2S_BGR_SPEC> {
        I2S_GATING_W::new(self, 2)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `i2s0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rst(&mut self, n: u8) -> I2S_RST_W<I2S_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        I2S_RST_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_rst(&mut self) -> I2S_RST_W<I2S_BGR_SPEC> {
        I2S_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_rst(&mut self) -> I2S_RST_W<I2S_BGR_SPEC> {
        I2S_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_rst(&mut self) -> I2S_RST_W<I2S_BGR_SPEC> {
        I2S_RST_W::new(self, 18)
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
#[doc = "I2S Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_BGR_SPEC;
impl crate::RegisterSpec for I2S_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_bgr::R`](R) reader structure"]
impl crate::Readable for I2S_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_bgr::W`](W) writer structure"]
impl crate::Writable for I2S_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_bgr to value 0"]
impl crate::Resettable for I2S_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
