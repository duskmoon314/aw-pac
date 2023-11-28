#[doc = "Register `twi_bgr` reader"]
pub type R = crate::R<TWI_BGR_SPEC>;
#[doc = "Register `twi_bgr` writer"]
pub type W = crate::W<TWI_BGR_SPEC>;
#[doc = "Field `twi_gating[0-3]` reader - Gating Clock"]
pub type TWI_GATING_R = crate::BitReader<TWI_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<TWI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: TWI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWI_GATING_A {
        match self.bits {
            false => TWI_GATING_A::MASK,
            true => TWI_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == TWI_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == TWI_GATING_A::PASS
    }
}
#[doc = "Field `twi_gating[0-3]` writer - Gating Clock"]
pub type TWI_GATING_W<'a, REG> = crate::BitWriter<'a, REG, TWI_GATING_A>;
impl<'a, REG> TWI_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_GATING_A::PASS)
    }
}
#[doc = "Field `twi_rst[0-3]` reader - Reset"]
pub type TWI_RST_R = crate::BitReader<TWI_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TWI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<TWI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TWI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TWI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TWI_RST_A {
        match self.bits {
            false => TWI_RST_A::ASSERT,
            true => TWI_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == TWI_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == TWI_RST_A::DEASSERT
    }
}
#[doc = "Field `twi_rst[0-3]` writer - Reset"]
pub type TWI_RST_W<'a, REG> = crate::BitWriter<'a, REG, TWI_RST_A>;
impl<'a, REG> TWI_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(TWI_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `twi0_gating` field"]
    #[inline(always)]
    pub fn twi_gating(&self, n: u8) -> TWI_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TWI_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn twi0_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn twi1_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn twi2_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    pub fn twi3_gating(&self) -> TWI_GATING_R {
        TWI_GATING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `twi0_rst` field"]
    #[inline(always)]
    pub fn twi_rst(&self, n: u8) -> TWI_RST_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TWI_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn twi0_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn twi1_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn twi2_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    pub fn twi3_rst(&self) -> TWI_RST_R {
        TWI_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `twi0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn twi_gating(&mut self, n: u8) -> TWI_GATING_W<TWI_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TWI_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi0_gating(&mut self) -> TWI_GATING_W<TWI_BGR_SPEC> {
        TWI_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi1_gating(&mut self) -> TWI_GATING_W<TWI_BGR_SPEC> {
        TWI_GATING_W::new(self, 1)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi2_gating(&mut self) -> TWI_GATING_W<TWI_BGR_SPEC> {
        TWI_GATING_W::new(self, 2)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn twi3_gating(&mut self) -> TWI_GATING_W<TWI_BGR_SPEC> {
        TWI_GATING_W::new(self, 3)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `twi0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn twi_rst(&mut self, n: u8) -> TWI_RST_W<TWI_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TWI_RST_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi0_rst(&mut self) -> TWI_RST_W<TWI_BGR_SPEC> {
        TWI_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi1_rst(&mut self) -> TWI_RST_W<TWI_BGR_SPEC> {
        TWI_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi2_rst(&mut self) -> TWI_RST_W<TWI_BGR_SPEC> {
        TWI_RST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn twi3_rst(&mut self) -> TWI_RST_W<TWI_BGR_SPEC> {
        TWI_RST_W::new(self, 19)
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
#[doc = "TWI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twi_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twi_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWI_BGR_SPEC;
impl crate::RegisterSpec for TWI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twi_bgr::R`](R) reader structure"]
impl crate::Readable for TWI_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twi_bgr::W`](W) writer structure"]
impl crate::Writable for TWI_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_bgr to value 0"]
impl crate::Resettable for TWI_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
