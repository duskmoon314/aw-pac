#[doc = "Register `dsp_bgr` reader"]
pub struct R(crate::R<DSP_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsp_bgr` writer"]
pub struct W(crate::W<DSP_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DSP_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_BGR_SPEC>) -> Self {
        W(writer)
    }
}
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
    pub fn variant(&self) -> CFG_GATING_A {
        match self.bits {
            false => CFG_GATING_A::MASK,
            true => CFG_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == CFG_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == CFG_GATING_A::PASS
    }
}
#[doc = "Field `cfg_gating` writer - Gating Clock"]
pub type CFG_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_BGR_SPEC, CFG_GATING_A, O>;
impl<'a, const O: u8> CFG_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(CFG_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
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
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::ASSERT,
            true => RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RST_A::DEASSERT
    }
}
#[doc = "Field `rst` writer - Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_BGR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
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
    pub fn variant(&self) -> CFG_RST_A {
        match self.bits {
            false => CFG_RST_A::ASSERT,
            true => CFG_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == CFG_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == CFG_RST_A::DEASSERT
    }
}
#[doc = "Field `cfg_rst` writer - Reset"]
pub type CFG_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_BGR_SPEC, CFG_RST_A, O>;
impl<'a, const O: u8> CFG_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(CFG_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
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
    pub fn variant(&self) -> DBG_RST_A {
        match self.bits {
            false => DBG_RST_A::ASSERT,
            true => DBG_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == DBG_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == DBG_RST_A::DEASSERT
    }
}
#[doc = "Field `dbg_rst` writer - Reset"]
pub type DBG_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_BGR_SPEC, DBG_RST_A, O>;
impl<'a, const O: u8> DBG_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(DBG_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
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
    pub fn cfg_gating(&mut self) -> CFG_GATING_W<1> {
        CFG_GATING_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<16> {
        RST_W::new(self)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_rst(&mut self) -> CFG_RST_W<17> {
        CFG_RST_W::new(self)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rst(&mut self) -> DBG_RST_W<18> {
        DBG_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_bgr](index.html) module"]
pub struct DSP_BGR_SPEC;
impl crate::RegisterSpec for DSP_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_bgr::R](R) reader structure"]
impl crate::Readable for DSP_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_bgr::W](W) writer structure"]
impl crate::Writable for DSP_BGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dsp_bgr to value 0"]
impl crate::Resettable for DSP_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
