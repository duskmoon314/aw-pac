#[doc = "Register `CCU_FAN_GATE` reader"]
pub struct R(crate::R<CCU_FAN_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCU_FAN_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCU_FAN_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCU_FAN_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCU_FAN_GATE` writer"]
pub struct W(crate::W<CCU_FAN_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCU_FAN_GATE_SPEC>;
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
impl From<crate::W<CCU_FAN_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCU_FAN_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating for CLK32K\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK32K_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK32K_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK32K_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK32K_EN` reader - Gating for CLK32K"]
pub struct CLK32K_EN_R(crate::FieldReader<bool, CLK32K_EN_A>);
impl CLK32K_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK32K_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK32K_EN_A {
        match self.bits {
            false => CLK32K_EN_A::OFF,
            true => CLK32K_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK32K_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK32K_EN_A::ON
    }
}
impl core::ops::Deref for CLK32K_EN_R {
    type Target = crate::FieldReader<bool, CLK32K_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK32K_EN` writer - Gating for CLK32K"]
pub struct CLK32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK32K_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK32K_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK32K_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK32K_EN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Gating for CLK25M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK25M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK25M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK25M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK25M_EN` reader - Gating for CLK25M"]
pub struct CLK25M_EN_R(crate::FieldReader<bool, CLK25M_EN_A>);
impl CLK25M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK25M_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK25M_EN_A {
        match self.bits {
            false => CLK25M_EN_A::OFF,
            true => CLK25M_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK25M_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK25M_EN_A::ON
    }
}
impl core::ops::Deref for CLK25M_EN_R {
    type Target = crate::FieldReader<bool, CLK25M_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK25M_EN` writer - Gating for CLK25M"]
pub struct CLK25M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK25M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK25M_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK25M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK25M_EN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Gating for CLK16M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK16M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK16M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK16M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK16M_EN` reader - Gating for CLK16M"]
pub struct CLK16M_EN_R(crate::FieldReader<bool, CLK16M_EN_A>);
impl CLK16M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK16M_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK16M_EN_A {
        match self.bits {
            false => CLK16M_EN_A::OFF,
            true => CLK16M_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK16M_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK16M_EN_A::ON
    }
}
impl core::ops::Deref for CLK16M_EN_R {
    type Target = crate::FieldReader<bool, CLK16M_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK16M_EN` writer - Gating for CLK16M"]
pub struct CLK16M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK16M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK16M_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK16M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK16M_EN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Gating for CLK12M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK12M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK12M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK12M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK12M_EN` reader - Gating for CLK12M"]
pub struct CLK12M_EN_R(crate::FieldReader<bool, CLK12M_EN_A>);
impl CLK12M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK12M_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK12M_EN_A {
        match self.bits {
            false => CLK12M_EN_A::OFF,
            true => CLK12M_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK12M_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK12M_EN_A::ON
    }
}
impl core::ops::Deref for CLK12M_EN_R {
    type Target = crate::FieldReader<bool, CLK12M_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK12M_EN` writer - Gating for CLK12M"]
pub struct CLK12M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK12M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK12M_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK12M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK12M_EN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Gating for CLK24M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK24M_EN_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<CLK24M_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLK24M_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK24M_EN` reader - Gating for CLK24M"]
pub struct CLK24M_EN_R(crate::FieldReader<bool, CLK24M_EN_A>);
impl CLK24M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLK24M_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK24M_EN_A {
        match self.bits {
            false => CLK24M_EN_A::OFF,
            true => CLK24M_EN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CLK24M_EN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CLK24M_EN_A::ON
    }
}
impl core::ops::Deref for CLK24M_EN_R {
    type Target = crate::FieldReader<bool, CLK24M_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK24M_EN` writer - Gating for CLK24M"]
pub struct CLK24M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK24M_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK24M_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CLK24M_EN_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CLK24M_EN_A::ON)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Gating for CLK32K"]
    #[inline(always)]
    pub fn clk32k_en(&self) -> CLK32K_EN_R {
        CLK32K_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Gating for CLK25M"]
    #[inline(always)]
    pub fn clk25m_en(&self) -> CLK25M_EN_R {
        CLK25M_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gating for CLK16M"]
    #[inline(always)]
    pub fn clk16m_en(&self) -> CLK16M_EN_R {
        CLK16M_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gating for CLK12M"]
    #[inline(always)]
    pub fn clk12m_en(&self) -> CLK12M_EN_R {
        CLK12M_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gating for CLK24M"]
    #[inline(always)]
    pub fn clk24m_en(&self) -> CLK24M_EN_R {
        CLK24M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Gating for CLK32K"]
    #[inline(always)]
    pub fn clk32k_en(&mut self) -> CLK32K_EN_W {
        CLK32K_EN_W { w: self }
    }
    #[doc = "Bit 3 - Gating for CLK25M"]
    #[inline(always)]
    pub fn clk25m_en(&mut self) -> CLK25M_EN_W {
        CLK25M_EN_W { w: self }
    }
    #[doc = "Bit 2 - Gating for CLK16M"]
    #[inline(always)]
    pub fn clk16m_en(&mut self) -> CLK16M_EN_W {
        CLK16M_EN_W { w: self }
    }
    #[doc = "Bit 1 - Gating for CLK12M"]
    #[inline(always)]
    pub fn clk12m_en(&mut self) -> CLK12M_EN_W {
        CLK12M_EN_W { w: self }
    }
    #[doc = "Bit 0 - Gating for CLK24M"]
    #[inline(always)]
    pub fn clk24m_en(&mut self) -> CLK24M_EN_W {
        CLK24M_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU FANOUT CLOCK GATE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccu_fan_gate](index.html) module"]
pub struct CCU_FAN_GATE_SPEC;
impl crate::RegisterSpec for CCU_FAN_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccu_fan_gate::R](R) reader structure"]
impl crate::Readable for CCU_FAN_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccu_fan_gate::W](W) writer structure"]
impl crate::Writable for CCU_FAN_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCU_FAN_GATE to value 0"]
impl crate::Resettable for CCU_FAN_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
