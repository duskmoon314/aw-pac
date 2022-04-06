#[doc = "Register `UART_BGR` reader"]
pub struct R(crate::R<UART_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_BGR` writer"]
pub struct W(crate::W<UART_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_BGR_SPEC>;
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
impl From<crate::W<UART_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<UART_RST_A> for bool {
    #[inline(always)]
    fn from(variant: UART_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `UART(0-5)_RST` reader - Reset"]
pub struct UART_RST_R(crate::FieldReader<bool, UART_RST_A>);
impl UART_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_RST_A {
        match self.bits {
            false => UART_RST_A::ASSERT,
            true => UART_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == UART_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == UART_RST_A::DEASSERT
    }
}
impl core::ops::Deref for UART_RST_R {
    type Target = crate::FieldReader<bool, UART_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `UART(0-5)_RST` writer - Reset"]
pub struct UART_RST_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> UART_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(UART_RST_A::DEASSERT)
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `UART(0-5)_RST` const generic writer - Reset"]
pub struct UART_RST_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> UART_RST_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(UART_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(UART_RST_A::DEASSERT)
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
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<UART_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: UART_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `UART(0-5)_GATING` reader - Gating Clock"]
pub struct UART_GATING_R(crate::FieldReader<bool, UART_GATING_A>);
impl UART_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_GATING_A {
        match self.bits {
            false => UART_GATING_A::MASK,
            true => UART_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == UART_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == UART_GATING_A::PASS
    }
}
impl core::ops::Deref for UART_GATING_R {
    type Target = crate::FieldReader<bool, UART_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `UART(0-5)_GATING` writer - Gating Clock"]
pub struct UART_GATING_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> UART_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(UART_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(UART_GATING_A::PASS)
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
        self.w.bits = (self.w.bits & !(1 << self.offset)) | ((value as u32 & 1) << self.offset);
        self.w
    }
}
#[doc = "Fields `UART(0-5)_GATING` const generic writer - Gating Clock"]
pub struct UART_GATING_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> UART_GATING_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(UART_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(UART_GATING_A::PASS)
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
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn uart_rst(&self, n: usize) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn uart0_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    pub fn uart3_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset"]
    #[inline(always)]
    pub fn uart4_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset"]
    #[inline(always)]
    pub fn uart5_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn uart_gating(&self, n: usize) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn uart0_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn uart1_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn uart2_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    pub fn uart3_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Gating Clock"]
    #[inline(always)]
    pub fn uart4_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Gating Clock"]
    #[inline(always)]
    pub fn uart5_gating(&self) -> UART_GATING_R {
        UART_GATING_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn uart_rst(&mut self, n: usize) -> UART_RST_W {
        UART_RST_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn uart0_rst(&mut self) -> UART_RST_CGW<16> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART_RST_CGW<17> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> UART_RST_CGW<18> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    pub fn uart3_rst(&mut self) -> UART_RST_CGW<19> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Bit 20 - Reset"]
    #[inline(always)]
    pub fn uart4_rst(&mut self) -> UART_RST_CGW<20> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Bit 21 - Reset"]
    #[inline(always)]
    pub fn uart5_rst(&mut self) -> UART_RST_CGW<21> {
        UART_RST_CGW { w: self }
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn uart_gating(&mut self, n: usize) -> UART_GATING_W {
        UART_GATING_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn uart0_gating(&mut self) -> UART_GATING_CGW<0> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn uart1_gating(&mut self) -> UART_GATING_CGW<1> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn uart2_gating(&mut self) -> UART_GATING_CGW<2> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    pub fn uart3_gating(&mut self) -> UART_GATING_CGW<3> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Bit 4 - Gating Clock"]
    #[inline(always)]
    pub fn uart4_gating(&mut self) -> UART_GATING_CGW<4> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Bit 5 - Gating Clock"]
    #[inline(always)]
    pub fn uart5_gating(&mut self) -> UART_GATING_CGW<5> {
        UART_GATING_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_bgr](index.html) module"]
pub struct UART_BGR_SPEC;
impl crate::RegisterSpec for UART_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_bgr::R](R) reader structure"]
impl crate::Readable for UART_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_bgr::W](W) writer structure"]
impl crate::Writable for UART_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_BGR to value 0"]
impl crate::Resettable for UART_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
