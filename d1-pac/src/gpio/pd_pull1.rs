#[doc = "Register `pd_pull1` reader"]
pub struct R(crate::R<PD_PULL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_PULL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_PULL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_PULL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_pull1` writer"]
pub struct W(crate::W<PD_PULL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_PULL1_SPEC>;
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
impl From<crate::W<PD_PULL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_PULL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pd_pull[16-22]` reader - PD Pull_up/down Select"]
pub type PD_PULL_R = crate::FieldReader<u8, PD_PULL_A>;
#[doc = "PD Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PD_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PD_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PD_PULL_A) -> Self {
        variant as _
    }
}
impl PD_PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_PULL_A {
        match self.bits {
            0 => PD_PULL_A::PULL_DISABLE,
            1 => PD_PULL_A::PULL_UP,
            2 => PD_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        *self == PD_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PD_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PD_PULL_A::PULL_DOWN
    }
}
#[doc = "Field `pd_pull[16-22]` writer - PD Pull_up/down Select"]
pub type PD_PULL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PD_PULL1_SPEC, u8, PD_PULL_A, 2, O>;
impl<'a, const O: u8> PD_PULL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PD_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PD_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PD_PULL_A::PULL_DOWN)
    }
}
impl R {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pd_pull(&self, n: u8) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> ((n - 16) * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd16_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd17_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd18_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd19_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd20_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd21_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd22_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn pd_pull<const O: u8>(&mut self) -> PD_PULL_W<O> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd16_pull(&mut self) -> PD_PULL_W<0> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd17_pull(&mut self) -> PD_PULL_W<2> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd18_pull(&mut self) -> PD_PULL_W<4> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd19_pull(&mut self) -> PD_PULL_W<6> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd20_pull(&mut self) -> PD_PULL_W<8> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd21_pull(&mut self) -> PD_PULL_W<10> {
        PD_PULL_W::new(self)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    #[must_use]
    pub fn pd22_pull(&mut self) -> PD_PULL_W<12> {
        PD_PULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Pull Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_pull1](index.html) module"]
pub struct PD_PULL1_SPEC;
impl crate::RegisterSpec for PD_PULL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_pull1::R](R) reader structure"]
impl crate::Readable for PD_PULL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_pull1::W](W) writer structure"]
impl crate::Writable for PD_PULL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pd_pull1 to value 0"]
impl crate::Resettable for PD_PULL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
