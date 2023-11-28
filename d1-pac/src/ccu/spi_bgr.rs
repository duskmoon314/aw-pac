#[doc = "Register `spi_bgr` reader"]
pub type R = crate::R<SPI_BGR_SPEC>;
#[doc = "Register `spi_bgr` writer"]
pub type W = crate::W<SPI_BGR_SPEC>;
#[doc = "Field `spi_gating[0-1]` reader - Gating Clock"]
pub type SPI_GATING_R = crate::BitReader<SPI_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<SPI_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI_GATING_A {
        match self.bits {
            false => SPI_GATING_A::MASK,
            true => SPI_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == SPI_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == SPI_GATING_A::PASS
    }
}
#[doc = "Field `spi_gating[0-1]` writer - Gating Clock"]
pub type SPI_GATING_W<'a, REG> = crate::BitWriter<'a, REG, SPI_GATING_A>;
impl<'a, REG> SPI_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_GATING_A::PASS)
    }
}
#[doc = "Field `spi_rst[0-1]` reader - Reset"]
pub type SPI_RST_R = crate::BitReader<SPI_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<SPI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPI_RST_A {
        match self.bits {
            false => SPI_RST_A::ASSERT,
            true => SPI_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SPI_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SPI_RST_A::DEASSERT
    }
}
#[doc = "Field `spi_rst[0-1]` writer - Reset"]
pub type SPI_RST_W<'a, REG> = crate::BitWriter<'a, REG, SPI_RST_A>;
impl<'a, REG> SPI_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `spi0_gating` field"]
    #[inline(always)]
    pub fn spi_gating(&self, n: u8) -> SPI_GATING_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SPI_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn spi0_gating(&self) -> SPI_GATING_R {
        SPI_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn spi1_gating(&self) -> SPI_GATING_R {
        SPI_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `spi0_rst` field"]
    #[inline(always)]
    pub fn spi_rst(&self, n: u8) -> SPI_RST_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SPI_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn spi0_rst(&self) -> SPI_RST_R {
        SPI_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn spi1_rst(&self) -> SPI_RST_R {
        SPI_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Gating Clock\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `spi0_gating` field"]
    #[inline(always)]
    #[must_use]
    pub fn spi_gating(&mut self, n: u8) -> SPI_GATING_W<SPI_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SPI_GATING_W::new(self, n)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_gating(&mut self) -> SPI_GATING_W<SPI_BGR_SPEC> {
        SPI_GATING_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_gating(&mut self) -> SPI_GATING_W<SPI_BGR_SPEC> {
        SPI_GATING_W::new(self, 1)
    }
    #[doc = "Reset\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `spi0_rst` field"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rst(&mut self, n: u8) -> SPI_RST_W<SPI_BGR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SPI_RST_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi0_rst(&mut self) -> SPI_RST_W<SPI_BGR_SPEC> {
        SPI_RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_rst(&mut self) -> SPI_RST_W<SPI_BGR_SPEC> {
        SPI_RST_W::new(self, 17)
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
#[doc = "SPI Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_BGR_SPEC;
impl crate::RegisterSpec for SPI_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_bgr::R`](R) reader structure"]
impl crate::Readable for SPI_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_bgr::W`](W) writer structure"]
impl crate::Writable for SPI_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_bgr to value 0"]
impl crate::Resettable for SPI_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
