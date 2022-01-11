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
#[doc = "Fields `PD(16-22)_PULL` reader - PD Pull_up/down Select"]
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
#[doc = "Fields `PD(16-22)_PULL` writer - PD Pull_up/down Select"]
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
        self.w.bits =
            (self.w.bits & !(0x03 << self.offset)) | ((value as u32 & 0x03) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pd_pull(&self, n: usize) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> (n - 16) * 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd16_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd17_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd18_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd19_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd20_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd21_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd22_pull(&self) -> PD_PULL_R {
        PD_PULL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "PD Pull_up/down Select"]
    #[inline(always)]
    pub unsafe fn pd_pull(&mut self, n: usize) -> PD_PULL_W {
        PD_PULL_W {
            w: self,
            offset: (n - 16) * 2,
        }
    }
    #[doc = "Bits 0:1 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd16_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W { w: self, offset: 0 }
    }
    #[doc = "Bits 2:3 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd17_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W { w: self, offset: 2 }
    }
    #[doc = "Bits 4:5 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd18_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W { w: self, offset: 4 }
    }
    #[doc = "Bits 6:7 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd19_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W { w: self, offset: 6 }
    }
    #[doc = "Bits 8:9 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd20_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W { w: self, offset: 8 }
    }
    #[doc = "Bits 10:11 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd21_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bits 12:13 - PD Pull_up/down Select"]
    #[inline(always)]
    pub fn pd22_pull(&mut self) -> PD_PULL_W {
        PD_PULL_W {
            w: self,
            offset: 12,
        }
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
}
#[doc = "`reset()` method sets pd_pull1 to value 0"]
impl crate::Resettable for PD_PULL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
