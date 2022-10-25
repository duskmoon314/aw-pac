#[doc = "Register `msgbox_bgr` reader"]
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
#[doc = "Register `msgbox_bgr` writer"]
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
#[doc = "Field `msgbox_gating[0-2]` reader - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub type MSGBOX_GATING_R = crate::BitReader<MSGBOX_GATING_A>;
#[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSGBOX_GATING_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSGBOX_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == MSGBOX_GATING_A::PASS
    }
}
#[doc = "Field `msgbox_gating[0-2]` writer - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
pub type MSGBOX_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MSGBOX_BGR_SPEC, MSGBOX_GATING_A, O>;
impl<'a, const O: u8> MSGBOX_GATING_W<'a, O> {
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
}
#[doc = "Field `msgbox_rst[0-2]` reader - CPU, DSP, RISC-V MSGBOX Reset"]
pub type MSGBOX_RST_R = crate::BitReader<MSGBOX_RST_A>;
#[doc = "CPU, DSP, RISC-V MSGBOX Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MSGBOX_RST_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MSGBOX_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == MSGBOX_RST_A::DEASSERT
    }
}
#[doc = "Field `msgbox_rst[0-2]` writer - CPU, DSP, RISC-V MSGBOX Reset"]
pub type MSGBOX_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MSGBOX_BGR_SPEC, MSGBOX_RST_A, O>;
impl<'a, const O: u8> MSGBOX_RST_W<'a, O> {
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
}
impl R {
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    pub unsafe fn msgbox_gating(&self, n: u8) -> MSGBOX_GATING_R {
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
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    pub unsafe fn msgbox_rst(&self, n: u8) -> MSGBOX_RST_R {
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
}
impl W {
    #[doc = "Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn msgbox_gating<const O: u8>(&mut self) -> MSGBOX_GATING_W<O> {
        MSGBOX_GATING_W::new(self)
    }
    #[doc = "Bit 0 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox0_gating(&mut self) -> MSGBOX_GATING_W<0> {
        MSGBOX_GATING_W::new(self)
    }
    #[doc = "Bit 1 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox1_gating(&mut self) -> MSGBOX_GATING_W<1> {
        MSGBOX_GATING_W::new(self)
    }
    #[doc = "Bit 2 - Gating Clock for CPU, DSP, RISC-V MSGBOX"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox2_gating(&mut self) -> MSGBOX_GATING_W<2> {
        MSGBOX_GATING_W::new(self)
    }
    #[doc = "CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn msgbox_rst<const O: u8>(&mut self) -> MSGBOX_RST_W<O> {
        MSGBOX_RST_W::new(self)
    }
    #[doc = "Bit 16 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox0_rst(&mut self) -> MSGBOX_RST_W<16> {
        MSGBOX_RST_W::new(self)
    }
    #[doc = "Bit 17 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox1_rst(&mut self) -> MSGBOX_RST_W<17> {
        MSGBOX_RST_W::new(self)
    }
    #[doc = "Bit 18 - CPU, DSP, RISC-V MSGBOX Reset"]
    #[inline(always)]
    #[must_use]
    pub fn msgbox2_rst(&mut self) -> MSGBOX_RST_W<18> {
        MSGBOX_RST_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msgbox_bgr to value 0"]
impl crate::Resettable for MSGBOX_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
