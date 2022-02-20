#[doc = "Register `TWI_EFR` reader"]
pub struct R(crate::R<TWI_EFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_EFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_EFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_EFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_EFR` writer"]
pub struct W(crate::W<TWI_EFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_EFR_SPEC>;
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
impl From<crate::W<TWI_EFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_EFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data Byte Number Follow Read Command Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBN_A {
    #[doc = "0: No data byte can be written after the read command"]
    B0 = 0,
    #[doc = "1: 1-byte data can be written after the read command"]
    B1 = 1,
    #[doc = "2: 2-byte data can be written after the read command"]
    B2 = 2,
    #[doc = "3: 3-byte data can be written after the read command"]
    B3 = 3,
}
impl From<DBN_A> for u8 {
    #[inline(always)]
    fn from(variant: DBN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dbn` reader - Data Byte Number Follow Read Command Control"]
pub struct DBN_R(crate::FieldReader<u8, DBN_A>);
impl DBN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBN_A {
        match self.bits {
            0 => DBN_A::B0,
            1 => DBN_A::B1,
            2 => DBN_A::B2,
            3 => DBN_A::B3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        **self == DBN_A::B0
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        **self == DBN_A::B1
    }
    #[doc = "Checks if the value of the field is `B2`"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        **self == DBN_A::B2
    }
    #[doc = "Checks if the value of the field is `B3`"]
    #[inline(always)]
    pub fn is_b3(&self) -> bool {
        **self == DBN_A::B3
    }
}
impl core::ops::Deref for DBN_R {
    type Target = crate::FieldReader<u8, DBN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbn` writer - Data Byte Number Follow Read Command Control"]
pub struct DBN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No data byte can be written after the read command"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut W {
        self.variant(DBN_A::B0)
    }
    #[doc = "1-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(DBN_A::B1)
    }
    #[doc = "2-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b2(self) -> &'a mut W {
        self.variant(DBN_A::B2)
    }
    #[doc = "3-byte data can be written after the read command"]
    #[inline(always)]
    pub fn b3(self) -> &'a mut W {
        self.variant(DBN_A::B3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    pub fn dbn(&mut self) -> DBN_W {
        DBN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Enhance Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_efr](index.html) module"]
pub struct TWI_EFR_SPEC;
impl crate::RegisterSpec for TWI_EFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_efr::R](R) reader structure"]
impl crate::Readable for TWI_EFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_efr::W](W) writer structure"]
impl crate::Writable for TWI_EFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_EFR to value 0"]
impl crate::Resettable for TWI_EFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
