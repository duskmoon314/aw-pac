#[doc = "Register `EMAC_ADDR_HIGH%s` reader"]
pub struct R(crate::R<EMAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_ADDR_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_ADDR_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_ADDR_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_ADDR_HIGH%s` writer"]
pub struct W(crate::W<EMAC_ADDR_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_ADDR_HIGH_SPEC>;
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
impl From<crate::W<EMAC_ADDR_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_ADDR_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "MAC Address Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_ADDR_CTL_A {
    #[doc = "0: `0`"]
    INVALID = 0,
    #[doc = "1: `1`"]
    VALID = 1,
}
impl From<MAC_ADDR_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_ADDR_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAC_ADDR_CTL` reader - MAC Address Valid"]
pub struct MAC_ADDR_CTL_R(crate::FieldReader<bool, MAC_ADDR_CTL_A>);
impl MAC_ADDR_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAC_ADDR_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAC_ADDR_CTL_A {
        match self.bits {
            false => MAC_ADDR_CTL_A::INVALID,
            true => MAC_ADDR_CTL_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == MAC_ADDR_CTL_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        **self == MAC_ADDR_CTL_A::VALID
    }
}
impl core::ops::Deref for MAC_ADDR_CTL_R {
    type Target = crate::FieldReader<bool, MAC_ADDR_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_ADDR_CTL` writer - MAC Address Valid"]
pub struct MAC_ADDR_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_ADDR_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_ADDR_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(MAC_ADDR_CTL_A::INVALID)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(MAC_ADDR_CTL_A::VALID)
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "MAC Address Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAC_ADDR_TYPE_A {
    #[doc = "0: `0`"]
    DST = 0,
    #[doc = "1: `1`"]
    SRC = 1,
}
impl From<MAC_ADDR_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: MAC_ADDR_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAC_ADDR_TYPE` reader - MAC Address Type"]
pub struct MAC_ADDR_TYPE_R(crate::FieldReader<bool, MAC_ADDR_TYPE_A>);
impl MAC_ADDR_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAC_ADDR_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAC_ADDR_TYPE_A {
        match self.bits {
            false => MAC_ADDR_TYPE_A::DST,
            true => MAC_ADDR_TYPE_A::SRC,
        }
    }
    #[doc = "Checks if the value of the field is `DST`"]
    #[inline(always)]
    pub fn is_dst(&self) -> bool {
        **self == MAC_ADDR_TYPE_A::DST
    }
    #[doc = "Checks if the value of the field is `SRC`"]
    #[inline(always)]
    pub fn is_src(&self) -> bool {
        **self == MAC_ADDR_TYPE_A::SRC
    }
}
impl core::ops::Deref for MAC_ADDR_TYPE_R {
    type Target = crate::FieldReader<bool, MAC_ADDR_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_ADDR_TYPE` writer - MAC Address Type"]
pub struct MAC_ADDR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_ADDR_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAC_ADDR_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dst(self) -> &'a mut W {
        self.variant(MAC_ADDR_TYPE_A::DST)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn src(self) -> &'a mut W {
        self.variant(MAC_ADDR_TYPE_A::SRC)
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `MAC_ADDR_BYTE_CTL` reader - MAC Address Byte Control Mask"]
pub struct MAC_ADDR_BYTE_CTL_R(crate::FieldReader<u8, u8>);
impl MAC_ADDR_BYTE_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAC_ADDR_BYTE_CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_ADDR_BYTE_CTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_ADDR_BYTE_CTL` writer - MAC Address Byte Control Mask"]
pub struct MAC_ADDR_BYTE_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_ADDR_BYTE_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `MAC_ADDR_HIGH` reader - "]
pub struct MAC_ADDR_HIGH_R(crate::FieldReader<u16, u16>);
impl MAC_ADDR_HIGH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAC_ADDR_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_ADDR_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_ADDR_HIGH` writer - "]
pub struct MAC_ADDR_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_ADDR_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - MAC Address Valid"]
    #[inline(always)]
    pub fn mac_addr_ctl(&self) -> MAC_ADDR_CTL_R {
        MAC_ADDR_CTL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - MAC Address Type"]
    #[inline(always)]
    pub fn mac_addr_type(&self) -> MAC_ADDR_TYPE_R {
        MAC_ADDR_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:29 - MAC Address Byte Control Mask"]
    #[inline(always)]
    pub fn mac_addr_byte_ctl(&self) -> MAC_ADDR_BYTE_CTL_R {
        MAC_ADDR_BYTE_CTL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_addr_high(&self) -> MAC_ADDR_HIGH_R {
        MAC_ADDR_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - MAC Address Valid"]
    #[inline(always)]
    pub fn mac_addr_ctl(&mut self) -> MAC_ADDR_CTL_W {
        MAC_ADDR_CTL_W { w: self }
    }
    #[doc = "Bit 30 - MAC Address Type"]
    #[inline(always)]
    pub fn mac_addr_type(&mut self) -> MAC_ADDR_TYPE_W {
        MAC_ADDR_TYPE_W { w: self }
    }
    #[doc = "Bits 24:29 - MAC Address Byte Control Mask"]
    #[inline(always)]
    pub fn mac_addr_byte_ctl(&mut self) -> MAC_ADDR_BYTE_CTL_W {
        MAC_ADDR_BYTE_CTL_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mac_addr_high(&mut self) -> MAC_ADDR_HIGH_W {
        MAC_ADDR_HIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC MAC Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_addr_high](index.html) module"]
pub struct EMAC_ADDR_HIGH_SPEC;
impl crate::RegisterSpec for EMAC_ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_addr_high::R](R) reader structure"]
impl crate::Readable for EMAC_ADDR_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_addr_high::W](W) writer structure"]
impl crate::Writable for EMAC_ADDR_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_ADDR_HIGH%s to value 0"]
impl crate::Resettable for EMAC_ADDR_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
