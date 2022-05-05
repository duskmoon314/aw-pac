#[doc = "Register `CIR_RXPCFG` reader"]
pub struct R(crate::R<CIR_RXPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_RXPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_RXPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_RXPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR_RXPCFG` writer"]
pub struct W(crate::W<CIR_RXPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_RXPCFG_SPEC>;
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
impl From<crate::W<CIR_RXPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_RXPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receiver Pulse Polarity Invert\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPPI_A {
    #[doc = "0: Do not invert receiver signal"]
    NOT_INVERT = 0,
    #[doc = "1: Invert receiver signal"]
    INVERT = 1,
}
impl From<RPPI_A> for bool {
    #[inline(always)]
    fn from(variant: RPPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPPI` reader - Receiver Pulse Polarity Invert"]
pub struct RPPI_R(crate::FieldReader<bool>);
impl RPPI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPPI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPPI_A {
        match self.bits {
            false => RPPI_A::NOT_INVERT,
            true => RPPI_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERT`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        **self == RPPI_A::NOT_INVERT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == RPPI_A::INVERT
    }
}
impl core::ops::Deref for RPPI_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPPI` writer - Receiver Pulse Polarity Invert"]
pub struct RPPI_W<'a> {
    w: &'a mut W,
}
impl<'a> RPPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPPI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not invert receiver signal"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut W {
        self.variant(RPPI_A::NOT_INVERT)
    }
    #[doc = "Invert receiver signal"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(RPPI_A::INVERT)
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Receiver Pulse Polarity Invert"]
    #[inline(always)]
    pub fn rppi(&self) -> RPPI_R {
        RPPI_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Receiver Pulse Polarity Invert"]
    #[inline(always)]
    pub fn rppi(&mut self) -> RPPI_W {
        RPPI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CIR Receiver Pulse Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir_rxpcfg](index.html) module"]
pub struct CIR_RXPCFG_SPEC;
impl crate::RegisterSpec for CIR_RXPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir_rxpcfg::R](R) reader structure"]
impl crate::Readable for CIR_RXPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir_rxpcfg::W](W) writer structure"]
impl crate::Writable for CIR_RXPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR_RXPCFG to value 0x04"]
impl crate::Resettable for CIR_RXPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
