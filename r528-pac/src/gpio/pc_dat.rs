#[doc = "Register `pc_dat` reader"]
pub struct R(crate::R<PC_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pc_dat` writer"]
pub struct W(crate::W<PC_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_DAT_SPEC>;
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
impl From<crate::W<PC_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC_DAT` reader - "]
pub struct PC_DAT_R(crate::FieldReader<u8, u8>);
impl PC_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PC_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC_DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC_DAT` writer - "]
pub struct PC_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_DAT_W<'a> {
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
    pub fn pc_dat(&self) -> PC_DAT_R {
        PC_DAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pc_dat(&mut self) -> PC_DAT_W {
        PC_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_dat](index.html) module"]
pub struct PC_DAT_SPEC;
impl crate::RegisterSpec for PC_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_dat::R](R) reader structure"]
impl crate::Readable for PC_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_dat::W](W) writer structure"]
impl crate::Writable for PC_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pc_dat to value 0"]
impl crate::Resettable for PC_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
