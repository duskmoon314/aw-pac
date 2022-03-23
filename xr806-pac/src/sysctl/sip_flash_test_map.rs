#[doc = "Register `SIP_FLASH_TEST_MAP` reader"]
pub struct R(crate::R<SIP_FLASH_TEST_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIP_FLASH_TEST_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIP_FLASH_TEST_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIP_FLASH_TEST_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIP_FLASH_TEST_MAP` writer"]
pub struct W(crate::W<SIP_FLASH_TEST_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIP_FLASH_TEST_MAP_SPEC>;
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
impl From<crate::W<SIP_FLASH_TEST_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIP_FLASH_TEST_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIP_FLASH_TEST_WRITE` reader - "]
pub struct SIP_FLASH_TEST_WRITE_R(crate::FieldReader<u16, u16>);
impl SIP_FLASH_TEST_WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SIP_FLASH_TEST_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIP_FLASH_TEST_WRITE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIP_FLASH_TEST_WRITE` writer - "]
pub struct SIP_FLASH_TEST_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIP_FLASH_TEST_WRITE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIP_FLASH_TEST_A {
    #[doc = "0: `0`"]
    MAPPING_DISABLED = 0,
    #[doc = "1: `1`"]
    MAPPED_TO_EXTERNAL_PINS = 1,
}
impl From<SIP_FLASH_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: SIP_FLASH_TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIP_FLASH_TEST` reader - "]
pub struct SIP_FLASH_TEST_R(crate::FieldReader<bool, SIP_FLASH_TEST_A>);
impl SIP_FLASH_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIP_FLASH_TEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIP_FLASH_TEST_A {
        match self.bits {
            false => SIP_FLASH_TEST_A::MAPPING_DISABLED,
            true => SIP_FLASH_TEST_A::MAPPED_TO_EXTERNAL_PINS,
        }
    }
    #[doc = "Checks if the value of the field is `MAPPING_DISABLED`"]
    #[inline(always)]
    pub fn is_mapping_disabled(&self) -> bool {
        **self == SIP_FLASH_TEST_A::MAPPING_DISABLED
    }
    #[doc = "Checks if the value of the field is `MAPPED_TO_EXTERNAL_PINS`"]
    #[inline(always)]
    pub fn is_mapped_to_external_pins(&self) -> bool {
        **self == SIP_FLASH_TEST_A::MAPPED_TO_EXTERNAL_PINS
    }
}
impl core::ops::Deref for SIP_FLASH_TEST_R {
    type Target = crate::FieldReader<bool, SIP_FLASH_TEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIP_FLASH_TEST` writer - "]
pub struct SIP_FLASH_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> SIP_FLASH_TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIP_FLASH_TEST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mapping_disabled(self) -> &'a mut W {
        self.variant(SIP_FLASH_TEST_A::MAPPING_DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mapped_to_external_pins(self) -> &'a mut W {
        self.variant(SIP_FLASH_TEST_A::MAPPED_TO_EXTERNAL_PINS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sip_flash_test_write(&self) -> SIP_FLASH_TEST_WRITE_R {
        SIP_FLASH_TEST_WRITE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sip_flash_test(&self) -> SIP_FLASH_TEST_R {
        SIP_FLASH_TEST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sip_flash_test_write(&mut self) -> SIP_FLASH_TEST_WRITE_W {
        SIP_FLASH_TEST_WRITE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sip_flash_test(&mut self) -> SIP_FLASH_TEST_W {
        SIP_FLASH_TEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SIP FLASH Test Mapping Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sip_flash_test_map](index.html) module"]
pub struct SIP_FLASH_TEST_MAP_SPEC;
impl crate::RegisterSpec for SIP_FLASH_TEST_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sip_flash_test_map::R](R) reader structure"]
impl crate::Readable for SIP_FLASH_TEST_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sip_flash_test_map::W](W) writer structure"]
impl crate::Writable for SIP_FLASH_TEST_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIP_FLASH_TEST_MAP to value 0"]
impl crate::Resettable for SIP_FLASH_TEST_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
