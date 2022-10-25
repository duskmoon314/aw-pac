#[doc = "Register `tv_io_tri` reader"]
pub struct R(crate::R<TV_IO_TRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_IO_TRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_IO_TRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_IO_TRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_io_tri` writer"]
pub struct W(crate::W<TV_IO_TRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_IO_TRI_SPEC>;
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
impl From<crate::W<TV_IO_TRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_IO_TRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_output_tri_en[0-3]` reader - IO\\[i\\] Output Trigger Enable"]
pub type IO_OUTPUT_TRI_EN_R = crate::BitReader<IO_OUTPUT_TRI_EN_A>;
#[doc = "IO\\[i\\] Output Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_OUTPUT_TRI_EN_A {
    #[doc = "1: Disable"]
    DISABLE = 1,
    #[doc = "0: Enable"]
    ENABLE = 0,
}
impl From<IO_OUTPUT_TRI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IO_OUTPUT_TRI_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_OUTPUT_TRI_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_OUTPUT_TRI_EN_A {
        match self.bits {
            true => IO_OUTPUT_TRI_EN_A::DISABLE,
            false => IO_OUTPUT_TRI_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IO_OUTPUT_TRI_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IO_OUTPUT_TRI_EN_A::ENABLE
    }
}
#[doc = "Field `io_output_tri_en[0-3]` writer - IO\\[i\\] Output Trigger Enable"]
pub type IO_OUTPUT_TRI_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TV_IO_TRI_SPEC, IO_OUTPUT_TRI_EN_A, O>;
impl<'a, const O: u8> IO_OUTPUT_TRI_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IO_OUTPUT_TRI_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IO_OUTPUT_TRI_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub unsafe fn io_output_tri_en(&self, n: u8) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io0_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io1_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io2_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    pub fn io3_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn io_output_tri_en<const O: u8>(&mut self) -> IO_OUTPUT_TRI_EN_W<O> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 24 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io0_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<24> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 25 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io1_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<25> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 26 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io2_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<26> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 27 - IO\\[i\\] Output Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn io3_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<27> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV SYNC Signal IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_io_tri](index.html) module"]
pub struct TV_IO_TRI_SPEC;
impl crate::RegisterSpec for TV_IO_TRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_io_tri::R](R) reader structure"]
impl crate::Readable for TV_IO_TRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_io_tri::W](W) writer structure"]
impl crate::Writable for TV_IO_TRI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_io_tri to value 0"]
impl crate::Resettable for TV_IO_TRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
