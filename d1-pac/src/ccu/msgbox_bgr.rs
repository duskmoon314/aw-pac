#[doc = "Register `MSGBOX_BGR` reader"]
pub struct R(crate::R<MSGBOX_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGBOX_BGR` writer"]
pub struct W(crate::W<MSGBOX_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_BGR_SPEC>;
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
impl From<crate::W<MSGBOX_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CPU, DSP, RISC-V MSGBOX Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGBOX_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<MSGBOX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MSGBOX_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `MSGBOX(0-2)_RST` reader - CPU, DSP, RISC-V MSGBOX Reset"]
pub struct MSGBOX_RST_R(crate::FieldReader<bool>);
impl MSGBOX_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSGBOX_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGBOX_RST_A {
        match self.bits {
            false => MSGBOX_RST_A::ASSERT,
            true => MSGBOX_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == MSGBOX_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == MSGBOX_RST_A::DEASSERT
    }
}
impl core::ops::Deref for MSGBOX_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `MSGBOX(0-2)_RST` const generic writer - CPU, DSP, RISC-V MSGBOX Reset"]
pub struct MSGBOX_RST_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> MSGBOX_RST_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGBOX_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(MSGBOX_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(MSGBOX_RST_A::DEASSERT)
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGBOX_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<MSGBOX_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: MSGBOX_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `MSGBOX(0-2)_GATING` reader - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub struct MSGBOX_GATING_R(crate::FieldReader<bool>);
impl MSGBOX_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSGBOX_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSGBOX_GATING_A {
        match self.bits {
            false => MSGBOX_GATING_A::MASK,
            true => MSGBOX_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == MSGBOX_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == MSGBOX_GATING_A::PASS
    }
}
impl core::ops::Deref for MSGBOX_GATING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `MSGBOX(0-2)_GATING` const generic writer - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub struct MSGBOX_GATING_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> MSGBOX_GATING_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGBOX_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MSGBOX_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(MSGBOX_GATING_A::PASS)
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl R {
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub unsafe fn msgbox_rst(&self, n: usize) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox0_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox1_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox2_rst(&self) -> MSGBOX_RST_R {
        MSGBOX_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub unsafe fn msgbox_gating(&self, n: usize) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox0_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox1_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox2_gating(&self) -> MSGBOX_GATING_R {
        MSGBOX_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub unsafe fn msgbox_rst<const O: usize>(&mut self) -> MSGBOX_RST_W<O> {
        MSGBOX_RST_W { w: self }
    }
    #[doc = "Bit 16 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox0_rst(&mut self) -> MSGBOX_RST_W<16> {
        MSGBOX_RST_W { w: self }
    }
    #[doc = "Bit 17 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox1_rst(&mut self) -> MSGBOX_RST_W<17> {
        MSGBOX_RST_W { w: self }
    }
    #[doc = "Bit 18 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub fn msgbox2_rst(&mut self) -> MSGBOX_RST_W<18> {
        MSGBOX_RST_W { w: self }
    }
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub unsafe fn msgbox_gating<const O: usize>(&mut self) -> MSGBOX_GATING_W<O> {
        MSGBOX_GATING_W { w: self }
    }
    #[doc = "Bit 0 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox0_gating(&mut self) -> MSGBOX_GATING_W<0> {
        MSGBOX_GATING_W { w: self }
    }
    #[doc = "Bit 1 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox1_gating(&mut self) -> MSGBOX_GATING_W<1> {
        MSGBOX_GATING_W { w: self }
    }
    #[doc = "Bit 2 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub fn msgbox2_gating(&mut self) -> MSGBOX_GATING_W<2> {
        MSGBOX_GATING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSGBOX Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_bgr](index.html) module"]
pub struct MSGBOX_BGR_SPEC;
impl crate::RegisterSpec for MSGBOX_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_bgr::R](R) reader structure"]
impl crate::Readable for MSGBOX_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_bgr::W](W) writer structure"]
impl crate::Writable for MSGBOX_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSGBOX_BGR to value 0"]
impl crate::Resettable for MSGBOX_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
