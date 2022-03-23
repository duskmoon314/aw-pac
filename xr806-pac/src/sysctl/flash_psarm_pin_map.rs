#[doc = "Register `FLASH_PSARM_PIN_MAP` reader"]
pub struct R(crate::R<FLASH_PSARM_PIN_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PSARM_PIN_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PSARM_PIN_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PSARM_PIN_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_PSARM_PIN_MAP` writer"]
pub struct W(crate::W<FLASH_PSARM_PIN_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PSARM_PIN_MAP_SPEC>;
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
impl From<crate::W<FLASH_PSARM_PIN_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PSARM_PIN_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode_5vbias` reader - "]
pub struct MODE_5VBIAS_R(crate::FieldReader<u8, u8>);
impl MODE_5VBIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_5VBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_5VBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode_5vbias` writer - "]
pub struct MODE_5VBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_5VBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASH_PSARM_PIN_MAP_A {
    #[doc = "1: `1`"]
    CS0 = 1,
    #[doc = "2: `10`"]
    CS1 = 2,
    #[doc = "3: `11`"]
    CS01 = 3,
}
impl From<FLASH_PSARM_PIN_MAP_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_PSARM_PIN_MAP_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `FLASH_PSARM_PIN_MAP(0-1)` reader - "]
pub struct FLASH_PSARM_PIN_MAP_R(crate::FieldReader<u8, FLASH_PSARM_PIN_MAP_A>);
impl FLASH_PSARM_PIN_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_PSARM_PIN_MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASH_PSARM_PIN_MAP_A> {
        match self.bits {
            1 => Some(FLASH_PSARM_PIN_MAP_A::CS0),
            2 => Some(FLASH_PSARM_PIN_MAP_A::CS1),
            3 => Some(FLASH_PSARM_PIN_MAP_A::CS01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CS0`"]
    #[inline(always)]
    pub fn is_cs0(&self) -> bool {
        **self == FLASH_PSARM_PIN_MAP_A::CS0
    }
    #[doc = "Checks if the value of the field is `CS1`"]
    #[inline(always)]
    pub fn is_cs1(&self) -> bool {
        **self == FLASH_PSARM_PIN_MAP_A::CS1
    }
    #[doc = "Checks if the value of the field is `CS01`"]
    #[inline(always)]
    pub fn is_cs01(&self) -> bool {
        **self == FLASH_PSARM_PIN_MAP_A::CS01
    }
}
impl core::ops::Deref for FLASH_PSARM_PIN_MAP_R {
    type Target = crate::FieldReader<u8, FLASH_PSARM_PIN_MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `FLASH_PSARM_PIN_MAP(0-1)` writer - "]
pub struct FLASH_PSARM_PIN_MAP_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> FLASH_PSARM_PIN_MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PSARM_PIN_MAP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cs0(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cs1(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn cs01(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << self.offset)) | ((value as u32 & 0x03) << self.offset);
        self.w
    }
}
#[doc = "Fields `FLASH_PSARM_PIN_MAP(0-1)` const generic writer - "]
pub struct FLASH_PSARM_PIN_MAP_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> FLASH_PSARM_PIN_MAP_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PSARM_PIN_MAP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cs0(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cs1(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn cs01(self) -> &'a mut W {
        self.variant(FLASH_PSARM_PIN_MAP_A::CS01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << O)) | ((value as u32 & 0x03) << O);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode_5vbias(&self) -> MODE_5VBIAS_R {
        MODE_5VBIAS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn flash_psarm_pin_map(&self, n: usize) -> FLASH_PSARM_PIN_MAP_R {
        FLASH_PSARM_PIN_MAP_R::new(((self.bits >> (n * 16)) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_psarm_pin_map0(&self) -> FLASH_PSARM_PIN_MAP_R {
        FLASH_PSARM_PIN_MAP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn flash_psarm_pin_map1(&self) -> FLASH_PSARM_PIN_MAP_R {
        FLASH_PSARM_PIN_MAP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode_5vbias(&mut self) -> MODE_5VBIAS_W {
        MODE_5VBIAS_W { w: self }
    }
    #[doc = ""]
    #[inline(always)]
    pub unsafe fn flash_psarm_pin_map(&mut self, n: usize) -> FLASH_PSARM_PIN_MAP_W {
        FLASH_PSARM_PIN_MAP_W {
            w: self,
            offset: n * 16,
        }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_psarm_pin_map0(&mut self) -> FLASH_PSARM_PIN_MAP_CGW<0> {
        FLASH_PSARM_PIN_MAP_CGW { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn flash_psarm_pin_map1(&mut self) -> FLASH_PSARM_PIN_MAP_CGW<16> {
        FLASH_PSARM_PIN_MAP_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH/PSRAM PIN MAPPING Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_psarm_pin_map](index.html) module"]
pub struct FLASH_PSARM_PIN_MAP_SPEC;
impl crate::RegisterSpec for FLASH_PSARM_PIN_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_psarm_pin_map::R](R) reader structure"]
impl crate::Readable for FLASH_PSARM_PIN_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_psarm_pin_map::W](W) writer structure"]
impl crate::Writable for FLASH_PSARM_PIN_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_PSARM_PIN_MAP to value 0"]
impl crate::Resettable for FLASH_PSARM_PIN_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
