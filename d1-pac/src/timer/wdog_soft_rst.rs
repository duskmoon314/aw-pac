#[doc = "Register `wdog_soft_rst` reader"]
pub type R = crate::R<WDOG_SOFT_RST_SPEC>;
#[doc = "Register `wdog_soft_rst` writer"]
pub type W = crate::W<WDOG_SOFT_RST_SPEC>;
#[doc = "Field `soft_rst_en` reader - Soft Reset Enable"]
pub type SOFT_RST_EN_R = crate::BitReader<SOFT_RST_EN_A>;
#[doc = "Soft Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFT_RST_EN_A {
    #[doc = "0: `0`"]
    DEASSERT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RST_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RST_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFT_RST_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOFT_RST_EN_A {
        match self.bits {
            false => SOFT_RST_EN_A::DEASSERT,
            true => SOFT_RST_EN_A::RESET,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SOFT_RST_EN_A::DEASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SOFT_RST_EN_A::RESET
    }
}
#[doc = "Field `soft_rst_en` writer - Soft Reset Enable"]
pub type SOFT_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG, SOFT_RST_EN_A>;
impl<'a, REG> SOFT_RST_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RST_EN_A::DEASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SOFT_RST_EN_A::RESET)
    }
}
#[doc = "Field `key_field` writer - Key Field"]
pub type KEY_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Soft Reset Enable"]
    #[inline(always)]
    pub fn soft_rst_en(&self) -> SOFT_RST_EN_R {
        SOFT_RST_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn soft_rst_en(&mut self) -> SOFT_RST_EN_W<WDOG_SOFT_RST_SPEC> {
        SOFT_RST_EN_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Key Field"]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<WDOG_SOFT_RST_SPEC> {
        KEY_FIELD_W::new(self, 16)
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
#[doc = "Watchdog Software Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_soft_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_soft_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_SOFT_RST_SPEC;
impl crate::RegisterSpec for WDOG_SOFT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_soft_rst::R`](R) reader structure"]
impl crate::Readable for WDOG_SOFT_RST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_soft_rst::W`](W) writer structure"]
impl crate::Writable for WDOG_SOFT_RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_soft_rst to value 0"]
impl crate::Resettable for WDOG_SOFT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
