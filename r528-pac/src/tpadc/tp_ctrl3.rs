#[doc = "Register `TP_CTRL3` reader"]
pub struct R(crate::R<TP_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_CTRL3` writer"]
pub struct W(crate::W<TP_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_CTRL3_SPEC>;
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
impl From<crate::W<TP_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<FILTER_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTER_EN` reader - Filter Enable"]
pub type FILTER_EN_R = crate::BitReader<FILTER_EN_A>;
impl FILTER_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_EN_A {
        match self.bits {
            false => FILTER_EN_A::DISABLE,
            true => FILTER_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTER_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FILTER_EN_A::ENABLE
    }
}
#[doc = "Field `FILTER_EN` writer - Filter Enable"]
pub type FILTER_EN_W<'a> = crate::BitWriter<'a, u32, TP_CTRL3_SPEC, FILTER_EN_A, 2>;
impl<'a> FILTER_EN_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTER_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FILTER_EN_A::ENABLE)
    }
}
#[doc = "Filter Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTER_TYPE_A {
    #[doc = "0: 4 / 2"]
    T42 = 0,
    #[doc = "1: 5 / 3"]
    T53 = 1,
    #[doc = "2: 8 / 4"]
    T84 = 2,
    #[doc = "3: 16 / 8"]
    T168 = 3,
}
impl From<FILTER_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FILTER_TYPE` reader - Filter Type"]
pub type FILTER_TYPE_R = crate::FieldReader<u8, FILTER_TYPE_A>;
impl FILTER_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_TYPE_A {
        match self.bits {
            0 => FILTER_TYPE_A::T42,
            1 => FILTER_TYPE_A::T53,
            2 => FILTER_TYPE_A::T84,
            3 => FILTER_TYPE_A::T168,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T42`"]
    #[inline(always)]
    pub fn is_t42(&self) -> bool {
        *self == FILTER_TYPE_A::T42
    }
    #[doc = "Checks if the value of the field is `T53`"]
    #[inline(always)]
    pub fn is_t53(&self) -> bool {
        *self == FILTER_TYPE_A::T53
    }
    #[doc = "Checks if the value of the field is `T84`"]
    #[inline(always)]
    pub fn is_t84(&self) -> bool {
        *self == FILTER_TYPE_A::T84
    }
    #[doc = "Checks if the value of the field is `T168`"]
    #[inline(always)]
    pub fn is_t168(&self) -> bool {
        *self == FILTER_TYPE_A::T168
    }
}
#[doc = "Field `FILTER_TYPE` writer - Filter Type"]
pub type FILTER_TYPE_W<'a> =
    crate::FieldWriterSafe<'a, u32, TP_CTRL3_SPEC, u8, FILTER_TYPE_A, 2, 0>;
impl<'a> FILTER_TYPE_W<'a> {
    #[doc = "4 / 2"]
    #[inline(always)]
    pub fn t42(self) -> &'a mut W {
        self.variant(FILTER_TYPE_A::T42)
    }
    #[doc = "5 / 3"]
    #[inline(always)]
    pub fn t53(self) -> &'a mut W {
        self.variant(FILTER_TYPE_A::T53)
    }
    #[doc = "8 / 4"]
    #[inline(always)]
    pub fn t84(self) -> &'a mut W {
        self.variant(FILTER_TYPE_A::T84)
    }
    #[doc = "16 / 8"]
    #[inline(always)]
    pub fn t168(self) -> &'a mut W {
        self.variant(FILTER_TYPE_A::T168)
    }
}
impl R {
    #[doc = "Bit 2 - Filter Enable"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Filter Type"]
    #[inline(always)]
    pub fn filter_type(&self) -> FILTER_TYPE_R {
        FILTER_TYPE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Filter Enable"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bits 0:1 - Filter Type"]
    #[inline(always)]
    pub fn filter_type(&mut self) -> FILTER_TYPE_W {
        FILTER_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_ctrl3](index.html) module"]
pub struct TP_CTRL3_SPEC;
impl crate::RegisterSpec for TP_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_ctrl3::R](R) reader structure"]
impl crate::Readable for TP_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_ctrl3::W](W) writer structure"]
impl crate::Writable for TP_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TP_CTRL3 to value 0"]
impl crate::Resettable for TP_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
