#[doc = "Register `HSK` reader"]
pub struct R(crate::R<HSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSK` writer"]
pub struct W(crate::W<HSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSK_SPEC>;
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
impl From<crate::W<HSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Handshake configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSK_A {
    #[doc = "165: `10100101`"]
    WAIT_CYCLE = 165,
    #[doc = "229: `11100101`"]
    HANDSHAKE = 229,
}
impl From<HSK_A> for u8 {
    #[inline(always)]
    fn from(variant: HSK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `hsk` reader - Handshake configuration"]
pub struct HSK_R(crate::FieldReader<u8>);
impl HSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HSK_A> {
        match self.bits {
            165 => Some(HSK_A::WAIT_CYCLE),
            229 => Some(HSK_A::HANDSHAKE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_CYCLE`"]
    #[inline(always)]
    pub fn is_wait_cycle(&self) -> bool {
        **self == HSK_A::WAIT_CYCLE
    }
    #[doc = "Checks if the value of the field is `HANDSHAKE`"]
    #[inline(always)]
    pub fn is_handshake(&self) -> bool {
        **self == HSK_A::HANDSHAKE
    }
}
impl core::ops::Deref for HSK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsk` writer - Handshake configuration"]
pub struct HSK_W<'a> {
    w: &'a mut W,
}
impl<'a> HSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10100101`"]
    #[inline(always)]
    pub fn wait_cycle(self) -> &'a mut W {
        self.variant(HSK_A::WAIT_CYCLE)
    }
    #[doc = "`11100101`"]
    #[inline(always)]
    pub fn handshake(self) -> &'a mut W {
        self.variant(HSK_A::HANDSHAKE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Handshake configuration"]
    #[inline(always)]
    pub fn hsk(&self) -> HSK_R {
        HSK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Handshake configuration"]
    #[inline(always)]
    pub fn hsk(&mut self) -> HSK_W {
        HSK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART DMA Handshake Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsk](index.html) module"]
pub struct HSK_SPEC;
impl crate::RegisterSpec for HSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsk::R](R) reader structure"]
impl crate::Readable for HSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsk::W](W) writer structure"]
impl crate::Writable for HSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSK to value 0"]
impl crate::Resettable for HSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
