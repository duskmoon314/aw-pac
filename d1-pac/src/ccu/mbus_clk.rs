#[doc = "Register `mbus_clk` reader"]
pub type R = crate::R<MBUS_CLK_SPEC>;
#[doc = "Register `mbus_clk` writer"]
pub type W = crate::W<MBUS_CLK_SPEC>;
#[doc = "Field `mbus_rst` reader - MBUS Reset"]
pub type MBUS_RST_R = crate::BitReader<MBUS_RST_A>;
#[doc = "MBUS Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MBUS_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<MBUS_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MBUS_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MBUS_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBUS_RST_A {
        match self.bits {
            false => MBUS_RST_A::ASSERT,
            true => MBUS_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == MBUS_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == MBUS_RST_A::DEASSERT
    }
}
#[doc = "Field `mbus_rst` writer - MBUS Reset"]
pub type MBUS_RST_W<'a, REG> = crate::BitWriter<'a, REG, MBUS_RST_A>;
impl<'a, REG> MBUS_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(MBUS_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(MBUS_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Bit 30 - MBUS Reset"]
    #[inline(always)]
    pub fn mbus_rst(&self) -> MBUS_RST_R {
        MBUS_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - MBUS Reset"]
    #[inline(always)]
    #[must_use]
    pub fn mbus_rst(&mut self) -> MBUS_RST_W<MBUS_CLK_SPEC> {
        MBUS_RST_W::new(self, 30)
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
#[doc = "MBUS Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mbus_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mbus_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MBUS_CLK_SPEC;
impl crate::RegisterSpec for MBUS_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbus_clk::R`](R) reader structure"]
impl crate::Readable for MBUS_CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mbus_clk::W`](W) writer structure"]
impl crate::Writable for MBUS_CLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mbus_clk to value 0"]
impl crate::Resettable for MBUS_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
