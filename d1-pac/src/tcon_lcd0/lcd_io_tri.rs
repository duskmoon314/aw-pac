#[doc = "Register `lcd_io_tri` reader"]
pub struct R(crate::R<LCD_IO_TRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_IO_TRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_IO_TRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_IO_TRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_io_tri` writer"]
pub struct W(crate::W<LCD_IO_TRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_IO_TRI_SPEC>;
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
impl From<crate::W<LCD_IO_TRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_IO_TRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_output_tri_en` reader - LCD output port D\\[23:0\\] output enable, with independent bit control."]
pub type DATA_OUTPUT_TRI_EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `data_output_tri_en` writer - LCD output port D\\[23:0\\] output enable, with independent bit control."]
pub type DATA_OUTPUT_TRI_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_IO_TRI_SPEC, u32, u32, 24, O>;
#[doc = "Field `io_output_tri_en[0-3]` reader - Enable the output of IO\\[i\\]"]
pub type IO_OUTPUT_TRI_EN_R = crate::BitReader<IO_OUTPUT_TRI_EN_A>;
#[doc = "Enable the output of IO\\[i\\]\n\nValue on reset: 1"]
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
#[doc = "Field `io_output_tri_en[0-3]` writer - Enable the output of IO\\[i\\]"]
pub type IO_OUTPUT_TRI_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_IO_TRI_SPEC, IO_OUTPUT_TRI_EN_A, O>;
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
#[doc = "Field `rgb_endian` reader - Set the endian of data bits"]
pub type RGB_ENDIAN_R = crate::BitReader<RGB_ENDIAN_A>;
#[doc = "Set the endian of data bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGB_ENDIAN_A {
    #[doc = "0: Normal"]
    NORMAL = 0,
    #[doc = "1: Bits_invert"]
    B_ITS_INVERT = 1,
}
impl From<RGB_ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: RGB_ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
impl RGB_ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_ENDIAN_A {
        match self.bits {
            false => RGB_ENDIAN_A::NORMAL,
            true => RGB_ENDIAN_A::B_ITS_INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RGB_ENDIAN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `B_ITS_INVERT`"]
    #[inline(always)]
    pub fn is_b_its_invert(&self) -> bool {
        *self == RGB_ENDIAN_A::B_ITS_INVERT
    }
}
#[doc = "Field `rgb_endian` writer - Set the endian of data bits"]
pub type RGB_ENDIAN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_IO_TRI_SPEC, RGB_ENDIAN_A, O>;
impl<'a, const O: u8> RGB_ENDIAN_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RGB_ENDIAN_A::NORMAL)
    }
    #[doc = "Bits_invert"]
    #[inline(always)]
    pub fn b_its_invert(self) -> &'a mut W {
        self.variant(RGB_ENDIAN_A::B_ITS_INVERT)
    }
}
impl R {
    #[doc = "Bits 0:23 - LCD output port D\\[23:0\\] output enable, with independent bit control."]
    #[inline(always)]
    pub fn data_output_tri_en(&self) -> DATA_OUTPUT_TRI_EN_R {
        DATA_OUTPUT_TRI_EN_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Enable the output of IO\\[i\\]"]
    #[inline(always)]
    pub unsafe fn io_output_tri_en(&self, n: u8) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    pub fn io0_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    pub fn io1_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    pub fn io2_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    pub fn io3_output_tri_en(&self) -> IO_OUTPUT_TRI_EN_R {
        IO_OUTPUT_TRI_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set the endian of data bits"]
    #[inline(always)]
    pub fn rgb_endian(&self) -> RGB_ENDIAN_R {
        RGB_ENDIAN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - LCD output port D\\[23:0\\] output enable, with independent bit control."]
    #[inline(always)]
    #[must_use]
    pub fn data_output_tri_en(&mut self) -> DATA_OUTPUT_TRI_EN_W<0> {
        DATA_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Enable the output of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn io_output_tri_en<const O: u8>(&mut self) -> IO_OUTPUT_TRI_EN_W<O> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 24 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io0_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<24> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 25 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io1_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<25> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 26 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io2_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<26> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 27 - Enable the output of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io3_output_tri_en(&mut self) -> IO_OUTPUT_TRI_EN_W<27> {
        IO_OUTPUT_TRI_EN_W::new(self)
    }
    #[doc = "Bit 28 - Set the endian of data bits"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_endian(&mut self) -> RGB_ENDIAN_W<28> {
        RGB_ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_io_tri](index.html) module"]
pub struct LCD_IO_TRI_SPEC;
impl crate::RegisterSpec for LCD_IO_TRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_io_tri::R](R) reader structure"]
impl crate::Readable for LCD_IO_TRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_io_tri::W](W) writer structure"]
impl crate::Writable for LCD_IO_TRI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_io_tri to value 0x0fff_ffff"]
impl crate::Resettable for LCD_IO_TRI_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
