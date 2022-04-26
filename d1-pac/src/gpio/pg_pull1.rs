#[doc = "Register `pg_pull1` reader"]
pub struct R(crate::R<PG_PULL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_PULL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_PULL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_PULL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_pull1` writer"]
pub struct W(crate::W<PG_PULL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_PULL1_SPEC>;
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
impl From<crate::W<PG_PULL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_PULL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PG Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PG_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PG_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `PG(16-18)_PULL` reader - PG Pull_up/down Select"]
pub struct PG_PULL_R(crate::FieldReader<u8>);
impl PG_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PG_PULL_A {
        match self.bits {
            0 => PG_PULL_A::PULL_DISABLE,
            1 => PG_PULL_A::PULL_UP,
            2 => PG_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PG_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PG_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PG_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PG_PULL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `PG(16-18)_PULL` const generic writer - PG Pull_up/down Select"]
pub struct PG_PULL_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PG_PULL_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PG_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PG_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PG_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << O)) | ((value as u32 & 3) << O);
        self.w
    }
}
impl R {
    #[doc = "PG Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pg_pull(&self, n: usize) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> ((n - 16) * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg16_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg17_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg18_pull(&self) -> PG_PULL_R {
        PG_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "PG Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pg_pull<const O: usize>(&mut self) -> PG_PULL_W<O> {
        PG_PULL_W { w: self }
    }
    #[doc = "Bits 0:1 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg16_pull(&mut self) -> PG_PULL_W<0> {
        PG_PULL_W { w: self }
    }
    #[doc = "Bits 2:3 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg17_pull(&mut self) -> PG_PULL_W<2> {
        PG_PULL_W { w: self }
    }
    #[doc = "Bits 4:5 - PG Pull_up/down Select"]
    #[inline(always)]
    pub fn pg18_pull(&mut self) -> PG_PULL_W<4> {
        PG_PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Pull Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_pull1](index.html) module"]
pub struct PG_PULL1_SPEC;
impl crate::RegisterSpec for PG_PULL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_pull1::R](R) reader structure"]
impl crate::Readable for PG_PULL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_pull1::W](W) writer structure"]
impl crate::Writable for PG_PULL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_pull1 to value 0"]
impl crate::Resettable for PG_PULL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
