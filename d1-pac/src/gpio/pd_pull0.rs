#[doc = "Register `pd_pull0` reader"]
pub struct R(crate::R<PD_PULL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_PULL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_PULL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_PULL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_pull0` writer"]
pub struct W(crate::W<PD_PULL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_PULL0_SPEC>;
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
impl From<crate::W<PD_PULL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_PULL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PD Pull_up/down Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Fields `PD(0-15)_PULL` reader - PD Pull_up/down Select"]
pub struct PD_PULL_R(crate::FieldReader<u8, PD_PULL_A>);
impl PD_PULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PD_PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PD_PULL_A::PULL_DISABLE
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == PD_PULL_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == PD_PULL_A::PULL_DOWN
    }
}
impl core::ops::Deref for PD_PULL_R {
    type Target = crate::FieldReader<u8, PD_PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `PD(0-15)_PULL` writer - PD Pull_up/down Select"]
pub struct PD_PULL_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PD_PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << self.offset)) | ((value as u32 & 3) << self.offset);
        self.w
    }
}
#[doc = "Fields `PD(0-15)_PULL` const generic writer - PD Pull_up/down Select"]
pub struct PD_PULL_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PD_PULL_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PD_PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << O)) | ((value as u32 & 3) << O);
        self.w
    }
}
impl R {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pd_pull(&self, n: usize) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd0_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd1_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd2_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd3_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd4_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd5_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd6_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd7_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd8_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd9_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd10_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd11_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd12_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd13_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd14_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd15_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pd_pull(&mut self, n: usize) -> PD_PULL_W {
        PD_PULL_W {
            w: self,
            offset: n * 2,
        }
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd0_pull(&mut self) -> PD_PULL_CGW<0> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd1_pull(&mut self) -> PD_PULL_CGW<2> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd2_pull(&mut self) -> PD_PULL_CGW<4> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd3_pull(&mut self) -> PD_PULL_CGW<6> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd4_pull(&mut self) -> PD_PULL_CGW<8> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd5_pull(&mut self) -> PD_PULL_CGW<10> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd6_pull(&mut self) -> PD_PULL_CGW<12> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 14:15 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd7_pull(&mut self) -> PD_PULL_CGW<14> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 16:17 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd8_pull(&mut self) -> PD_PULL_CGW<16> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 18:19 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd9_pull(&mut self) -> PD_PULL_CGW<18> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 20:21 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd10_pull(&mut self) -> PD_PULL_CGW<20> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 22:23 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd11_pull(&mut self) -> PD_PULL_CGW<22> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 24:25 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd12_pull(&mut self) -> PD_PULL_CGW<24> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 26:27 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd13_pull(&mut self) -> PD_PULL_CGW<26> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 28:29 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd14_pull(&mut self) -> PD_PULL_CGW<28> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Bits 30:31 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd15_pull(&mut self) -> PD_PULL_CGW<30> {
        PD_PULL_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PD Pull Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_pull0](index.html) module"]
pub struct PD_PULL0_SPEC;
impl crate::RegisterSpec for PD_PULL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_pull0::R](R) reader structure"]
impl crate::Readable for PD_PULL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_pull0::W](W) writer structure"]
impl crate::Writable for PD_PULL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_pull0 to value 0"]
impl crate::Resettable for PD_PULL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
