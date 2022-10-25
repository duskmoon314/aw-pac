#[doc = "Register `csic_bist_cs` reader"]
pub struct R(crate::R<CSIC_BIST_CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_BIST_CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_BIST_CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_BIST_CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_bist_cs` writer"]
pub struct W(crate::W<CSIC_BIST_CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_BIST_CS_SPEC>;
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
impl From<crate::W<CSIC_BIST_CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_BIST_CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bist_cs` reader - "]
pub type BIST_CS_R = crate::FieldReader<u8, BIST_CS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BIST_CS_A {
    #[doc = "0: Set when BK0 memory bist"]
    SET_BK0 = 0,
    #[doc = "1: Set when BK1 memory bist"]
    SET_BK1 = 1,
}
impl From<BIST_CS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIST_CS_A) -> Self {
        variant as _
    }
}
impl BIST_CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BIST_CS_A> {
        match self.bits {
            0 => Some(BIST_CS_A::SET_BK0),
            1 => Some(BIST_CS_A::SET_BK1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET_BK0`"]
    #[inline(always)]
    pub fn is_set_bk0(&self) -> bool {
        *self == BIST_CS_A::SET_BK0
    }
    #[doc = "Checks if the value of the field is `SET_BK1`"]
    #[inline(always)]
    pub fn is_set_bk1(&self) -> bool {
        *self == BIST_CS_A::SET_BK1
    }
}
#[doc = "Field `bist_cs` writer - "]
pub type BIST_CS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_BIST_CS_SPEC, u8, BIST_CS_A, 3, O>;
impl<'a, const O: u8> BIST_CS_W<'a, O> {
    #[doc = "Set when BK0 memory bist"]
    #[inline(always)]
    pub fn set_bk0(self) -> &'a mut W {
        self.variant(BIST_CS_A::SET_BK0)
    }
    #[doc = "Set when BK1 memory bist"]
    #[inline(always)]
    pub fn set_bk1(self) -> &'a mut W {
        self.variant(BIST_CS_A::SET_BK1)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bist_cs(&self) -> BIST_CS_R {
        BIST_CS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn bist_cs(&mut self) -> BIST_CS_W<0> {
        BIST_CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC BIST CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_bist_cs](index.html) module"]
pub struct CSIC_BIST_CS_SPEC;
impl crate::RegisterSpec for CSIC_BIST_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_bist_cs::R](R) reader structure"]
impl crate::Readable for CSIC_BIST_CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_bist_cs::W](W) writer structure"]
impl crate::Writable for CSIC_BIST_CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_cs to value 0"]
impl crate::Resettable for CSIC_BIST_CS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
