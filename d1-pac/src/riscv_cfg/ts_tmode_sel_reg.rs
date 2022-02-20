#[doc = "Register `TS_TMODE_SEL_REG` reader"]
pub struct R(crate::R<TS_TMODE_SEL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TS_TMODE_SEL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TS_TMODE_SEL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TS_TMODE_SEL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TS_TMODE_SEL_REG` writer"]
pub struct W(crate::W<TS_TMODE_SEL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TS_TMODE_SEL_REG_SPEC>;
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
impl From<crate::W<TS_TMODE_SEL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TS_TMODE_SEL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timestamp Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS_TEST_MODE_EN_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    TEST = 1,
}
impl From<TS_TEST_MODE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TS_TEST_MODE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS_TEST_MODE_EN` reader - Timestamp Test Mode Enable"]
pub struct TS_TEST_MODE_EN_R(crate::FieldReader<bool, TS_TEST_MODE_EN_A>);
impl TS_TEST_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TS_TEST_MODE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS_TEST_MODE_EN_A {
        match self.bits {
            false => TS_TEST_MODE_EN_A::NORMAL,
            true => TS_TEST_MODE_EN_A::TEST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TS_TEST_MODE_EN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        **self == TS_TEST_MODE_EN_A::TEST
    }
}
impl core::ops::Deref for TS_TEST_MODE_EN_R {
    type Target = crate::FieldReader<bool, TS_TEST_MODE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_TEST_MODE_EN` writer - Timestamp Test Mode Enable"]
pub struct TS_TEST_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_TEST_MODE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS_TEST_MODE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TS_TEST_MODE_EN_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(TS_TEST_MODE_EN_A::TEST)
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
    #[doc = "Bit 0 - Timestamp Test Mode Enable"]
    #[inline(always)]
    pub fn ts_test_mode_en(&self) -> TS_TEST_MODE_EN_R {
        TS_TEST_MODE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Test Mode Enable"]
    #[inline(always)]
    pub fn ts_test_mode_en(&mut self) -> TS_TEST_MODE_EN_W {
        TS_TEST_MODE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Test Mode Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ts_tmode_sel_reg](index.html) module"]
pub struct TS_TMODE_SEL_REG_SPEC;
impl crate::RegisterSpec for TS_TMODE_SEL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ts_tmode_sel_reg::R](R) reader structure"]
impl crate::Readable for TS_TMODE_SEL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ts_tmode_sel_reg::W](W) writer structure"]
impl crate::Writable for TS_TMODE_SEL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TS_TMODE_SEL_REG to value 0"]
impl crate::Resettable for TS_TMODE_SEL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
