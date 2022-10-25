#[doc = "Register `twi_efr` reader"]
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
#[doc = "Register `twi_efr` writer"]
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
#[doc = "Field `dbn` reader - Data Byte Number Follow Read Command Control"]
pub type DBN_R = crate::FieldReader<u8, DBN_A>;
#[doc = "Data Byte Number Follow Read Command Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DBN_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DBN_A::B0
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DBN_A::B1
    }
    #[doc = "Checks if the value of the field is `B2`"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        *self == DBN_A::B2
    }
    #[doc = "Checks if the value of the field is `B3`"]
    #[inline(always)]
    pub fn is_b3(&self) -> bool {
        *self == DBN_A::B3
    }
}
#[doc = "Field `dbn` writer - Data Byte Number Follow Read Command Control"]
pub type DBN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TWI_EFR_SPEC, u8, DBN_A, 2, O>;
impl<'a, const O: u8> DBN_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    pub fn dbn(&self) -> DBN_R {
        DBN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Byte Number Follow Read Command Control"]
    #[inline(always)]
    #[must_use]
    pub fn dbn(&mut self) -> DBN_W<0> {
        DBN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_efr to value 0"]
impl crate::Resettable for TWI_EFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
