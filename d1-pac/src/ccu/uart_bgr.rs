#[doc = "Register `uart_bgr` reader"]
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
#[doc = "Register `uart_bgr` writer"]
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
#[doc = "Field `uart_gating[0-5]` reader - Gating Clock"]
pub type UART_GATING_R = crate::BitReader<UART_GATING_A>;
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UART_GATING_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UART_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == UART_GATING_A::PASS
    }
}
#[doc = "Field `uart_gating[0-5]` writer - Gating Clock"]
pub type UART_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_BGR_SPEC, UART_GATING_A, O>;
impl<'a, const O: u8> UART_GATING_W<'a, O> {
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
}
#[doc = "Field `uart_rst[0-5]` reader - Reset"]
pub type UART_RST_R = crate::BitReader<UART_RST_A>;
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl UART_RST_R {
    #[doc = "Get enumerated values variant"]
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
        *self == UART_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == UART_RST_A::DEASSERT
    }
}
#[doc = "Field `uart_rst[0-5]` writer - Reset"]
pub type UART_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_BGR_SPEC, UART_RST_A, O>;
impl<'a, const O: u8> UART_RST_W<'a, O> {
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
}
impl R {
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn uart_gating(&self, n: u8) -> UART_GATING_R {
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
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn uart_rst(&self, n: u8) -> UART_RST_R {
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
}
impl W {
    #[doc = "Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn uart_gating<const O: u8>(&mut self) -> UART_GATING_W<O> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_gating(&mut self) -> UART_GATING_W<0> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_gating(&mut self) -> UART_GATING_W<1> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_gating(&mut self) -> UART_GATING_W<2> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 3 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_gating(&mut self) -> UART_GATING_W<3> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 4 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_gating(&mut self) -> UART_GATING_W<4> {
        UART_GATING_W::new(self)
    }
    #[doc = "Bit 5 - Gating Clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart5_gating(&mut self) -> UART_GATING_W<5> {
        UART_GATING_W::new(self)
    }
    #[doc = "Reset"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn uart_rst<const O: u8>(&mut self) -> UART_RST_W<O> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart0_rst(&mut self) -> UART_RST_W<16> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1_rst(&mut self) -> UART_RST_W<17> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_rst(&mut self) -> UART_RST_W<18> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 19 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_rst(&mut self) -> UART_RST_W<19> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 20 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_rst(&mut self) -> UART_RST_W<20> {
        UART_RST_W::new(self)
    }
    #[doc = "Bit 21 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart5_rst(&mut self) -> UART_RST_W<21> {
        UART_RST_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_bgr to value 0"]
impl crate::Resettable for UART_BGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
