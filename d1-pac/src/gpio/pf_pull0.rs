#[doc = "Register `pf_pull0` reader"]
pub struct R(crate::R<PF_PULL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_PULL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_PULL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_PULL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_pull0` writer"]
pub struct W(crate::W<PF_PULL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_PULL0_SPEC>;
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
impl From<crate::W<PF_PULL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_PULL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PF Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PF_PULL_A {
    #[doc = "0: `0`"]
    PULL_DISABLE = 0,
    #[doc = "1: `1`"]
    PULL_UP = 1,
    #[doc = "2: `10`"]
    PULL_DOWN = 2,
}
impl From<PF_PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PF_PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `PF(0-6)_PULL` reader - PF Pull_up/down Select"]
pub struct PF_PULL_R(crate::FieldReader<u8, PF_PULL_A>);
impl PF_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF_PULL_A {
        match self.bits {
            0 => PF_PULL_A::PULL_DISABLE,
            1 => PF_PULL_A::PULL_UP,
            2 => PF_PULL_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_DISABLE`"]
    #[inline(always)]
    pub fn is_pull_disable(&self) -> bool {
        **self == PF_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PF_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PF_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PF_PULL_R {
    type Target = crate::FieldReader<u8, PF_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `PF(0-6)_PULL` writer - PF Pull_up/down Select"]
pub struct PF_PULL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PF_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << self.offset)) | ((value as u32 & 3) << self.offset);
        self.w
    }
}
#[doc = "Fields `PF(0-6)_PULL` const generic writer - PF Pull_up/down Select"]
pub struct PF_PULL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PF_PULL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pull_disable(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_UP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PF_PULL_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << O)) | ((value as u32 & 3) << O);
        self.w
    }
}
impl R {
    #[doc = "PF Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pf_pull(&self, n: usize) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf0_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf1_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf2_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf3_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf4_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf5_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf6_pull(&self) -> PF_PULL_R {
        PF_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "PF Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pf_pull(&mut self, n: usize) -> PF_PULL_W {
        PF_PULL_W {
            w: self,
            offset: n * 2,
        }
    }
    #[doc = "Bits 0:1 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf0_pull(&mut self) -> PF_PULL_CGW<0> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 2:3 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf1_pull(&mut self) -> PF_PULL_CGW<2> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 4:5 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf2_pull(&mut self) -> PF_PULL_CGW<4> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 6:7 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf3_pull(&mut self) -> PF_PULL_CGW<6> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 8:9 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf4_pull(&mut self) -> PF_PULL_CGW<8> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 10:11 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf5_pull(&mut self) -> PF_PULL_CGW<10> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Bits 12:13 - PF Pull_up/down Select"]
    #[inline(always)]
    pub fn pf6_pull(&mut self) -> PF_PULL_CGW<12> {
        PF_PULL_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Pull Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_pull0](index.html) module"]
pub struct PF_PULL0_SPEC;
impl crate::RegisterSpec for PF_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_pull0::R](R) reader structure"]
impl crate::Readable for PF_PULL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_pull0::W](W) writer structure"]
impl crate::Writable for PF_PULL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pf_pull0 to value 0"]
impl crate::Resettable for PF_PULL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
