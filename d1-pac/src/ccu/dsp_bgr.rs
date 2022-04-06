#[doc = "Register `DSP_BGR` reader"]
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
#[doc = "Register `DSP_BGR` writer"]
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
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DBG_RST` reader - Reset"]
pub struct DBG_RST_R(crate::FieldReader<bool, DBG_RST_A>);
impl DBG_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DBG_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == DBG_RST_A::DEASSERT
    }
}
impl core::ops::Deref for DBG_RST_R {
    type Target = crate::FieldReader<bool, DBG_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_RST` writer - Reset"]
pub struct DBG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CFG_RST` reader - Reset"]
pub struct CFG_RST_R(crate::FieldReader<bool, CFG_RST_A>);
impl CFG_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CFG_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == CFG_RST_A::DEASSERT
    }
}
impl core::ops::Deref for CFG_RST_R {
    type Target = crate::FieldReader<bool, CFG_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_RST` writer - Reset"]
pub struct CFG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RST` reader - Reset"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == RST_A::DEASSERT
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Reset"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CFG_GATING` reader - Gating Clock"]
pub struct CFG_GATING_R(crate::FieldReader<bool, CFG_GATING_A>);
impl CFG_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CFG_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == CFG_GATING_A::PASS
    }
}
impl core::ops::Deref for CFG_GATING_R {
    type Target = crate::FieldReader<bool, CFG_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG_GATING` writer - Gating Clock"]
pub struct CFG_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn dbg_rst(&self) -> DBG_RST_R {
        DBG_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn cfg_rst(&self) -> CFG_RST_R {
        CFG_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn cfg_gating(&self) -> CFG_GATING_R {
        CFG_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn dbg_rst(&mut self) -> DBG_RST_W {
        DBG_RST_W { w: self }
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn cfg_rst(&mut self) -> CFG_RST_W {
        CFG_RST_W { w: self }
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn cfg_gating(&mut self) -> CFG_GATING_W {
        CFG_GATING_W { w: self }
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
}
#[doc = "`reset()` method sets DSP_BGR to value 0"]
impl crate::Resettable for DSP_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
