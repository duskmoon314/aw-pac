#[doc = "Register `CCU_FAN` reader"]
pub struct R(crate::R<CCU_FAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_FAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_FAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_FAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCU_FAN` writer"]
pub struct W(crate::W<CCU_FAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_FAN_SPEC>;
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
impl From<crate::W<CCU_FAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_FAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating for CLK_FANOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_FANOUT_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK_FANOUT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK_FANOUT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `CLK_FANOUT(0-2)_EN` reader - Gating for CLK_FANOUT"]
pub struct CLK_FANOUT_EN_R(crate::FieldReader<bool, CLK_FANOUT_EN_A>);
impl CLK_FANOUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK_FANOUT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_FANOUT_EN_A {
        match self.bits {
            false => CLK_FANOUT_EN_A::OFF,
            true => CLK_FANOUT_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK_FANOUT_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK_FANOUT_EN_A::ON
    }
}
impl core::ops::Deref for CLK_FANOUT_EN_R {
    type Target = crate::FieldReader<bool, CLK_FANOUT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CLK_FANOUT(0-2)_EN` writer - Gating for CLK_FANOUT"]
pub struct CLK_FANOUT_EN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CLK_FANOUT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_FANOUT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_FANOUT_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_FANOUT_EN_A::ON)
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
#[doc = "Fields `CLK_FANOUT(0-2)_EN` const generic writer - Gating for CLK_FANOUT"]
pub struct CLK_FANOUT_EN_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> CLK_FANOUT_EN_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_FANOUT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK_FANOUT_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK_FANOUT_EN_A::ON)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_FANOUT_SEL_A {
    #[doc = "0: `0`"]
    CLK32K = 0,
    #[doc = "1: `1`"]
    CLK12M = 1,
    #[doc = "2: `10`"]
    CLK16M = 2,
    #[doc = "3: `11`"]
    CLK24M = 3,
    #[doc = "4: `100`"]
    CLK25M = 4,
    #[doc = "5: `101`"]
    CLK27M = 5,
    #[doc = "6: `110`"]
    PCLK = 6,
}
impl From<CLK_FANOUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_FANOUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `CLK_FANOUT(0-2)_SEL` reader - "]
pub struct CLK_FANOUT_SEL_R(crate::FieldReader<u8, CLK_FANOUT_SEL_A>);
impl CLK_FANOUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_FANOUT_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_FANOUT_SEL_A> {
        match self.bits {
            0 => Some(CLK_FANOUT_SEL_A::CLK32K),
            1 => Some(CLK_FANOUT_SEL_A::CLK12M),
            2 => Some(CLK_FANOUT_SEL_A::CLK16M),
            3 => Some(CLK_FANOUT_SEL_A::CLK24M),
            4 => Some(CLK_FANOUT_SEL_A::CLK25M),
            5 => Some(CLK_FANOUT_SEL_A::CLK27M),
            6 => Some(CLK_FANOUT_SEL_A::PCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK32K`"]
    #[inline(always)]
    pub fn is_clk32k(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK32K
    }
    #[doc = "Checks if the value of the field is `CLK12M`"]
    #[inline(always)]
    pub fn is_clk12m(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK12M
    }
    #[doc = "Checks if the value of the field is `CLK16M`"]
    #[inline(always)]
    pub fn is_clk16m(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK16M
    }
    #[doc = "Checks if the value of the field is `CLK24M`"]
    #[inline(always)]
    pub fn is_clk24m(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK24M
    }
    #[doc = "Checks if the value of the field is `CLK25M`"]
    #[inline(always)]
    pub fn is_clk25m(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK25M
    }
    #[doc = "Checks if the value of the field is `CLK27M`"]
    #[inline(always)]
    pub fn is_clk27m(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::CLK27M
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == CLK_FANOUT_SEL_A::PCLK
    }
}
impl core::ops::Deref for CLK_FANOUT_SEL_R {
    type Target = crate::FieldReader<u8, CLK_FANOUT_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `CLK_FANOUT(0-2)_SEL` writer - "]
pub struct CLK_FANOUT_SEL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> CLK_FANOUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_FANOUT_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK32K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk12m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK12M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK16M)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn clk24m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK24M)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clk25m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK25M)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk27m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK27M)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::PCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << self.offset)) | ((value as u32 & 7) << self.offset);
        self.w
    }
}
#[doc = "Fields `CLK_FANOUT(0-2)_SEL` const generic writer - "]
pub struct CLK_FANOUT_SEL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> CLK_FANOUT_SEL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_FANOUT_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn clk32k(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK32K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clk12m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK12M)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn clk16m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK16M)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn clk24m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK24M)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn clk25m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK25M)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn clk27m(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::CLK27M)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CLK_FANOUT_SEL_A::PCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << O)) | ((value as u32 & 7) << O);
        self.w
    }
}
impl R {
    #[doc = "Gating for CLK_FANOUT"]
    #[inline(always)]
    pub unsafe fn clk_fanout_en(&self, n: usize) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> (n + 21)) & 1) != 0)
    }
    #[doc = "Bit 21 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout0_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout1_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout2_en(&self) -> CLK_FANOUT_EN_R {
        CLK_FANOUT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn clk_fanout_sel(&self, n: usize) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_fanout0_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn clk_fanout1_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn clk_fanout2_sel(&self) -> CLK_FANOUT_SEL_R {
        CLK_FANOUT_SEL_R::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Gating for CLK_FANOUT"]
    #[inline(always)]
    pub unsafe fn clk_fanout_en(&mut self, n: usize) -> CLK_FANOUT_EN_W {
        CLK_FANOUT_EN_W {
            w: self,
            offset: n + 21,
        }
    }
    #[doc = "Bit 21 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout0_en(&mut self) -> CLK_FANOUT_EN_CGW<21> {
        CLK_FANOUT_EN_CGW { w: self }
    }
    #[doc = "Bit 22 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout1_en(&mut self) -> CLK_FANOUT_EN_CGW<22> {
        CLK_FANOUT_EN_CGW { w: self }
    }
    #[doc = "Bit 23 - Gating for CLK_FANOUT"]
    #[inline(always)]
    pub fn clk_fanout2_en(&mut self) -> CLK_FANOUT_EN_CGW<23> {
        CLK_FANOUT_EN_CGW { w: self }
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn clk_fanout_sel(&mut self, n: usize) -> CLK_FANOUT_SEL_W {
        CLK_FANOUT_SEL_W {
            w: self,
            offset: n * 3,
        }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clk_fanout0_sel(&mut self) -> CLK_FANOUT_SEL_CGW<0> {
        CLK_FANOUT_SEL_CGW { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn clk_fanout1_sel(&mut self) -> CLK_FANOUT_SEL_CGW<3> {
        CLK_FANOUT_SEL_CGW { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn clk_fanout2_sel(&mut self) -> CLK_FANOUT_SEL_CGW<6> {
        CLK_FANOUT_SEL_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU FANOUT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_fan](index.html) module"]
pub struct CCU_FAN_SPEC;
impl crate::RegisterSpec for CCU_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_fan::R](R) reader structure"]
impl crate::Readable for CCU_FAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_fan::W](W) writer structure"]
impl crate::Writable for CCU_FAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCU_FAN to value 0"]
impl crate::Resettable for CCU_FAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
