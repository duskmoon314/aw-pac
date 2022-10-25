#[doc = "Register `pe_pull1` reader"]
pub struct R(crate::R<PE_PULL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_PULL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE_PULL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE_PULL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pe_pull1` writer"]
pub struct W(crate::W<PE_PULL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_PULL1_SPEC>;
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
impl From<crate::W<PE_PULL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE_PULL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pe_pull[16-17]` reader - PE Pull_up/down Select"]
pub type PE_PULL_R = crate::FieldReader<u8, PE_PULL_A>;
#[doc = "PE Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PE_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PE_PULL_A) -> Self {
        variant as _
    }
}
impl PE_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_PULL_A {
        match self.bits {
            0 => PE_PULL_A::PULL_DISABLE,
            1 => PE_PULL_A::PULL_UP,
            2 => PE_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PE_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PE_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PE_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pe_pull[16-17]` writer - PE Pull_up/down Select"]
pub type PE_PULL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PE_PULL1_SPEC, u8, PE_PULL_A, 2, O>;
impl<'a, const O: u8> PE_PULL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PE_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PE_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PE_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PE Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pe_pull(&self, n: u8) -> PE_PULL_R {
        PE_PULL_R::new(((self.bits >> ((n - 16) * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PE Pull_up/down Select"]
    #[inline(always)]
    pub fn pe16_pull(&self) -> PE_PULL_R {
        PE_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PE Pull_up/down Select"]
    #[inline(always)]
    pub fn pe17_pull(&self) -> PE_PULL_R {
        PE_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "PE Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pe_pull<const O: u8>(&mut self) -> PE_PULL_W<O> {
        PE_PULL_W::new(self)
    }
    #[doc = "Bits 0:1 - PE Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe16_pull(&mut self) -> PE_PULL_W<0> {
        PE_PULL_W::new(self)
    }
    #[doc = "Bits 2:3 - PE Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe17_pull(&mut self) -> PE_PULL_W<2> {
        PE_PULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE Pull Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_pull1](index.html) module"]
pub struct PE_PULL1_SPEC;
impl crate::RegisterSpec for PE_PULL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_pull1::R](R) reader structure"]
impl crate::Readable for PE_PULL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_pull1::W](W) writer structure"]
impl crate::Writable for PE_PULL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_pull1 to value 0"]
impl crate::Resettable for PE_PULL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
