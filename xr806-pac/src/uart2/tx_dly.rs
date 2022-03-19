#[doc = "Register `TX_DLY` reader"]
pub struct R(crate::R<TX_DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_DLY` writer"]
pub struct W(crate::W<TX_DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_DLY_SPEC>;
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
impl From<crate::W<TX_DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - "]
pub struct DLY_R(crate::FieldReader<u8, u8>);
impl DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLY` writer - "]
pub struct DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dly(&mut self) -> DLY_W {
        DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART TX Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_dly](index.html) module"]
pub struct TX_DLY_SPEC;
impl crate::RegisterSpec for TX_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_dly::R](R) reader structure"]
impl crate::Readable for TX_DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_dly::W](W) writer structure"]
impl crate::Writable for TX_DLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_DLY to value 0"]
impl crate::Resettable for TX_DLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
