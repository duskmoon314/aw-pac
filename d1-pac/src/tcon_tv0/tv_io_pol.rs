#[doc = "Register `tv_io_pol` reader"]
pub struct R(crate::R<TV_IO_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_IO_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_IO_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_IO_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_io_pol` writer"]
pub struct W(crate::W<TV_IO_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_IO_POL_SPEC>;
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
impl From<crate::W<TV_IO_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_IO_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_inv[0-3]` reader - IO\\[i\\] Invert"]
pub type IO_INV_R = crate::BitReader<IO_INV_A>;
#[doc = "IO\\[i\\] Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_INV_A {
    #[doc = "0: Not invert"]
    NOT_INVERT = 0,
    #[doc = "1: Invert"]
    INVERT = 1,
}
impl From<IO_INV_A> for bool {
    #[inline(always)]
    fn from(variant: IO_INV_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_INV_A {
        match self.bits {
            false => IO_INV_A::NOT_INVERT,
            true => IO_INV_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERT`"]
    #[inline(always)]
    pub fn is_not_invert(&self) -> bool {
        *self == IO_INV_A::NOT_INVERT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == IO_INV_A::INVERT
    }
}
#[doc = "Field `io_inv[0-3]` writer - IO\\[i\\] Invert"]
pub type IO_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TV_IO_POL_SPEC, IO_INV_A, O>;
impl<'a, const O: u8> IO_INV_W<'a, O> {
    #[doc = "Not invert"]
    #[inline(always)]
    pub fn not_invert(self) -> &'a mut W {
        self.variant(IO_INV_A::NOT_INVERT)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(IO_INV_A::INVERT)
    }
}
impl R {
    #[doc = "IO\\[i\\] Invert"]
    #[inline(always)]
    pub unsafe fn io_inv(&self, n: u8) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io0_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io1_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io2_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IO\\[i\\] Invert"]
    #[inline(always)]
    pub fn io3_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn io_inv<const O: u8>(&mut self) -> IO_INV_W<O> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 24 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io0_inv(&mut self) -> IO_INV_W<24> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 25 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io1_inv(&mut self) -> IO_INV_W<25> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 26 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io2_inv(&mut self) -> IO_INV_W<26> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 27 - IO\\[i\\] Invert"]
    #[inline(always)]
    #[must_use]
    pub fn io3_inv(&mut self) -> IO_INV_W<27> {
        IO_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV SYNC Signal Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_io_pol](index.html) module"]
pub struct TV_IO_POL_SPEC;
impl crate::RegisterSpec for TV_IO_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_io_pol::R](R) reader structure"]
impl crate::Readable for TV_IO_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_io_pol::W](W) writer structure"]
impl crate::Writable for TV_IO_POL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_io_pol to value 0"]
impl crate::Resettable for TV_IO_POL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
