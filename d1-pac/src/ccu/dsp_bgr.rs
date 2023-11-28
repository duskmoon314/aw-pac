#[doc = "Register `dsp_bgr` reader"]
pub type R = crate::R<DSP_BGR_SPEC>;
#[doc = "Register `dsp_bgr` writer"]
pub type W = crate::W<DSP_BGR_SPEC>;
#[doc = "Field `cfg_gating` reader - Gating Clock"]
pub type CFG_GATING_R = crate::BitReader<CFG_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<CFG_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_GATING_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFG_GATING_A {
        match self.bits {
            false => CFG_GATING_A::MASK,
            true => CFG_GATING_A::PASS,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == CFG_GATING_A::MASK
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CFG_GATING_A::PASS
    }
}
#[doc = "Field `cfg_gating` writer - Gating Clock"]
pub type CFG_GATING_W<'a, REG> = crate::BitWriter<'a, REG, CFG_GATING_A>;
impl<'a, REG> CFG_GATING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_GATING_A::PASS)
    }
}
#[doc = "Field `rst` reader - Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::ASSERT,
            true => RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RST_A::DEASSERT
    }
}
#[doc = "Field `rst` writer - Reset"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG, RST_A>;
impl<'a, REG> RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(RST_A::DEASSERT)
    }
}
#[doc = "Field `cfg_rst` reader - Reset"]
pub type CFG_RST_R = crate::BitReader<CFG_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFG_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<CFG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CFG_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CFG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFG_RST_A {
        match self.bits {
            false => CFG_RST_A::ASSERT,
            true => CFG_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == CFG_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == CFG_RST_A::DEASSERT
    }
}
#[doc = "Field `cfg_rst` writer - Reset"]
pub type CFG_RST_W<'a, REG> = crate::BitWriter<'a, REG, CFG_RST_A>;
impl<'a, REG> CFG_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(CFG_RST_A::DEASSERT)
    }
}
#[doc = "Field `dbg_rst` reader - Reset"]
pub type DBG_RST_R = crate::BitReader<DBG_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<DBG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_RST_A {
        match self.bits {
            false => DBG_RST_A::ASSERT,
            true => DBG_RST_A::DEASSERT,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == DBG_RST_A::ASSERT
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == DBG_RST_A::DEASSERT
    }
}
#[doc = "Field `dbg_rst` writer - Reset"]
pub type DBG_RST_W<'a, REG> = crate::BitWriter<'a, REG, DBG_RST_A>;
impl<'a, REG> DBG_RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_RST_A::DEASSERT)
    }
}
impl R {
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn cfg_gating(&self) -> CFG_GATING_R {
        CFG_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn cfg_rst(&self) -> CFG_RST_R {
        CFG_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn dbg_rst(&self) -> DBG_RST_R {
        DBG_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_gating(&mut self) -> CFG_GATING_W<DSP_BGR_SPEC> {
        CFG_GATING_W::new(self, 1)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<DSP_BGR_SPEC> {
        RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_rst(&mut self) -> CFG_RST_W<DSP_BGR_SPEC> {
        CFG_RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rst(&mut self) -> DBG_RST_W<DSP_BGR_SPEC> {
        DBG_RST_W::new(self, 18)
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
#[doc = "DSP Bus Gating Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsp_bgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsp_bgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSP_BGR_SPEC;
impl crate::RegisterSpec for DSP_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsp_bgr::R`](R) reader structure"]
impl crate::Readable for DSP_BGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsp_bgr::W`](W) writer structure"]
impl crate::Writable for DSP_BGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dsp_bgr to value 0"]
impl crate::Resettable for DSP_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
