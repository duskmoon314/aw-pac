#[doc = "Register `wdog_cfg` reader"]
pub type R = crate::R<WDOG_CFG_SPEC>;
#[doc = "Register `wdog_cfg` writer"]
pub type W = crate::W<WDOG_CFG_SPEC>;
#[doc = "Field `wdog_mode` reader - Configure the operating mode for the watchdog"]
pub type WDOG_MODE_R = crate::FieldReader<WDOG_MODE_A>;
#[doc = "Configure the operating mode for the watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOG_MODE_A {
    #[doc = "1: `1`"]
    WHOLD_SYSTEM = 1,
    #[doc = "2: `10`"]
    ONLY_INTERRUPT = 2,
}
impl From<WDOG_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOG_MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDOG_MODE_A {
    type Ux = u8;
}
impl WDOG_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDOG_MODE_A> {
        match self.bits {
            1 => Some(WDOG_MODE_A::WHOLD_SYSTEM),
            2 => Some(WDOG_MODE_A::ONLY_INTERRUPT),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_whold_system(&self) -> bool {
        *self == WDOG_MODE_A::WHOLD_SYSTEM
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_only_interrupt(&self) -> bool {
        *self == WDOG_MODE_A::ONLY_INTERRUPT
    }
}
#[doc = "Field `wdog_mode` writer - Configure the operating mode for the watchdog"]
pub type WDOG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WDOG_MODE_A>;
impl<'a, REG> WDOG_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn whold_system(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_MODE_A::WHOLD_SYSTEM)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn only_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_MODE_A::ONLY_INTERRUPT)
    }
}
#[doc = "Field `wdog_clk_src` reader - Select the clock source for the watchdog."]
pub type WDOG_CLK_SRC_R = crate::BitReader<WDOG_CLK_SRC_A>;
#[doc = "Select the clock source for the watchdog.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG_CLK_SRC_A {
    #[doc = "0: `0`"]
    HOSC_32K = 0,
    #[doc = "1: `1`"]
    LOSC_32K = 1,
}
impl From<WDOG_CLK_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_CLK_SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG_CLK_SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDOG_CLK_SRC_A {
        match self.bits {
            false => WDOG_CLK_SRC_A::HOSC_32K,
            true => WDOG_CLK_SRC_A::LOSC_32K,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_hosc_32k(&self) -> bool {
        *self == WDOG_CLK_SRC_A::HOSC_32K
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_losc_32k(&self) -> bool {
        *self == WDOG_CLK_SRC_A::LOSC_32K
    }
}
#[doc = "Field `wdog_clk_src` writer - Select the clock source for the watchdog."]
pub type WDOG_CLK_SRC_W<'a, REG> = crate::BitWriter<'a, REG, WDOG_CLK_SRC_A>;
impl<'a, REG> WDOG_CLK_SRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn hosc_32k(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_CLK_SRC_A::HOSC_32K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn losc_32k(self) -> &'a mut crate::W<REG> {
        self.variant(WDOG_CLK_SRC_A::LOSC_32K)
    }
}
#[doc = "Field `key_field` writer - Key Field"]
pub type KEY_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Configure the operating mode for the watchdog"]
    #[inline(always)]
    pub fn wdog_mode(&self) -> WDOG_MODE_R {
        WDOG_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Select the clock source for the watchdog."]
    #[inline(always)]
    pub fn wdog_clk_src(&self) -> WDOG_CLK_SRC_R {
        WDOG_CLK_SRC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the operating mode for the watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_mode(&mut self) -> WDOG_MODE_W<WDOG_CFG_SPEC> {
        WDOG_MODE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Select the clock source for the watchdog."]
    #[inline(always)]
    #[must_use]
    pub fn wdog_clk_src(&mut self) -> WDOG_CLK_SRC_W<WDOG_CFG_SPEC> {
        WDOG_CLK_SRC_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Key Field"]
    #[inline(always)]
    #[must_use]
    pub fn key_field(&mut self) -> KEY_FIELD_W<WDOG_CFG_SPEC> {
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
#[doc = "Watchdog Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdog_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdog_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOG_CFG_SPEC;
impl crate::RegisterSpec for WDOG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_cfg::R`](R) reader structure"]
impl crate::Readable for WDOG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdog_cfg::W`](W) writer structure"]
impl crate::Writable for WDOG_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wdog_cfg to value 0"]
impl crate::Resettable for WDOG_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
