#[doc = "Register `pg_eint_status` reader"]
pub struct R(crate::R<PG_EINT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_EINT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_EINT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_EINT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_eint_status` writer"]
pub struct W(crate::W<PG_EINT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_EINT_STATUS_SPEC>;
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
impl From<crate::W<PG_EINT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_EINT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External INT Pending Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT_STATUS_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<EINT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EINT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `EINT(0-17)_STATUS` reader - External INT Pending Bit"]
pub struct EINT_STATUS_R(crate::FieldReader<bool, EINT_STATUS_A>);
impl EINT_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EINT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT_STATUS_A {
        match self.bits {
            false => EINT_STATUS_A::NO_PENDING,
            true => EINT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        **self == EINT_STATUS_A::NO_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == EINT_STATUS_A::PENDING
    }
}
impl core::ops::Deref for EINT_STATUS_R {
    type Target = crate::FieldReader<bool, EINT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `EINT(0-17)_STATUS` writer - External INT Pending Bit"]
pub struct EINT_STATUS_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> EINT_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT_STATUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut W {
        self.variant(EINT_STATUS_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(EINT_STATUS_A::PENDING)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
impl R {
    #[doc = "External INT Pending Bit"]
    #[inline(always)]
    pub unsafe fn eint_status(&self, n: usize) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint0_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint1_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint2_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint3_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint4_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint5_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint6_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint7_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint8_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint9_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint10_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint11_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint12_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint13_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint14_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint15_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint16_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint17_status(&self) -> EINT_STATUS_R {
        EINT_STATUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "External INT Pending Bit"]
    #[inline(always)]
    pub unsafe fn eint_status(&mut self, n: usize) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint0_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 0 }
    }
    #[doc = "Bit 1 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint1_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint2_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint3_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint4_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint5_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint6_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 6 }
    }
    #[doc = "Bit 7 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint7_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint8_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 8 }
    }
    #[doc = "Bit 9 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint9_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W { w: self, offset: 9 }
    }
    #[doc = "Bit 10 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint10_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint11_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint12_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint13_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint14_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint15_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 15,
        }
    }
    #[doc = "Bit 16 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint16_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 16,
        }
    }
    #[doc = "Bit 17 - External INT Pending Bit"]
    #[inline(always)]
    pub fn eint17_status(&mut self) -> EINT_STATUS_W {
        EINT_STATUS_W {
            w: self,
            offset: 17,
        }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG External Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_eint_status](index.html) module"]
pub struct PG_EINT_STATUS_SPEC;
impl crate::RegisterSpec for PG_EINT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_eint_status::R](R) reader structure"]
impl crate::Readable for PG_EINT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_eint_status::W](W) writer structure"]
impl crate::Writable for PG_EINT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_eint_status to value 0"]
impl crate::Resettable for PG_EINT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
