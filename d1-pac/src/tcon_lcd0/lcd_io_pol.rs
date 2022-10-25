#[doc = "Register `lcd_io_pol` reader"]
pub struct R(crate::R<LCD_IO_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_IO_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_IO_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_IO_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lcd_io_pol` writer"]
pub struct W(crate::W<LCD_IO_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_IO_POL_SPEC>;
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
impl From<crate::W<LCD_IO_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_IO_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_inv` reader - LCD output port D\\[23:0\\] polarity control, with independent bit control."]
pub type DATA_INV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `data_inv` writer - LCD output port D\\[23:0\\] polarity control, with independent bit control."]
pub type DATA_INV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_IO_POL_SPEC, u32, u32, 24, O>;
#[doc = "Field `io_inv[0-3]` reader - Enable invert function of IO\\[i\\]"]
pub type IO_INV_R = crate::BitReader<IO_INV_A>;
#[doc = "Enable invert function of IO\\[i\\]\n\nValue on reset: 0"]
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
#[doc = "Field `io_inv[0-3]` writer - Enable invert function of IO\\[i\\]"]
pub type IO_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCD_IO_POL_SPEC, IO_INV_A, O>;
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
#[doc = "Field `dclk_sel` reader - Set the phase offset of clock and data in hv mode."]
pub type DCLK_SEL_R = crate::FieldReader<u8, DCLK_SEL_A>;
#[doc = "Set the phase offset of clock and data in hv mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCLK_SEL_A {
    #[doc = "0: Used DCLK0 (normal phase offset)"]
    DCLK0 = 0,
    #[doc = "1: Used DCLK1 (1/3 phase offset)"]
    DCLK1 = 1,
    #[doc = "2: Used DCLK2 (2/3 phase offset)"]
    DCLK2 = 2,
    #[doc = "4: DCLK0/2 phase 0"]
    DCLK02_0 = 4,
    #[doc = "5: DCLK0/2 phase 90"]
    DCLK02_90 = 5,
}
impl From<DCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCLK_SEL_A) -> Self {
        variant as _
    }
}
impl DCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCLK_SEL_A> {
        match self.bits {
            0 => Some(DCLK_SEL_A::DCLK0),
            1 => Some(DCLK_SEL_A::DCLK1),
            2 => Some(DCLK_SEL_A::DCLK2),
            4 => Some(DCLK_SEL_A::DCLK02_0),
            5 => Some(DCLK_SEL_A::DCLK02_90),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DCLK0`"]
    #[inline(always)]
    pub fn is_dclk0(&self) -> bool {
        *self == DCLK_SEL_A::DCLK0
    }
    #[doc = "Checks if the value of the field is `DCLK1`"]
    #[inline(always)]
    pub fn is_dclk1(&self) -> bool {
        *self == DCLK_SEL_A::DCLK1
    }
    #[doc = "Checks if the value of the field is `DCLK2`"]
    #[inline(always)]
    pub fn is_dclk2(&self) -> bool {
        *self == DCLK_SEL_A::DCLK2
    }
    #[doc = "Checks if the value of the field is `DCLK02_0`"]
    #[inline(always)]
    pub fn is_dclk02_0(&self) -> bool {
        *self == DCLK_SEL_A::DCLK02_0
    }
    #[doc = "Checks if the value of the field is `DCLK02_90`"]
    #[inline(always)]
    pub fn is_dclk02_90(&self) -> bool {
        *self == DCLK_SEL_A::DCLK02_90
    }
}
#[doc = "Field `dclk_sel` writer - Set the phase offset of clock and data in hv mode."]
pub type DCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LCD_IO_POL_SPEC, u8, DCLK_SEL_A, 3, O>;
impl<'a, const O: u8> DCLK_SEL_W<'a, O> {
    #[doc = "Used DCLK0 (normal phase offset)"]
    #[inline(always)]
    pub fn dclk0(self) -> &'a mut W {
        self.variant(DCLK_SEL_A::DCLK0)
    }
    #[doc = "Used DCLK1 (1/3 phase offset)"]
    #[inline(always)]
    pub fn dclk1(self) -> &'a mut W {
        self.variant(DCLK_SEL_A::DCLK1)
    }
    #[doc = "Used DCLK2 (2/3 phase offset)"]
    #[inline(always)]
    pub fn dclk2(self) -> &'a mut W {
        self.variant(DCLK_SEL_A::DCLK2)
    }
    #[doc = "DCLK0/2 phase 0"]
    #[inline(always)]
    pub fn dclk02_0(self) -> &'a mut W {
        self.variant(DCLK_SEL_A::DCLK02_0)
    }
    #[doc = "DCLK0/2 phase 90"]
    #[inline(always)]
    pub fn dclk02_90(self) -> &'a mut W {
        self.variant(DCLK_SEL_A::DCLK02_90)
    }
}
#[doc = "Field `io_output_sel` reader - When it is set as '1', the d\\[23:0\\], io0, io1, io3 are sync to dclk."]
pub type IO_OUTPUT_SEL_R = crate::BitReader<IO_OUTPUT_SEL_A>;
#[doc = "When it is set as '1', the d\\[23:0\\], io0, io1, io3 are sync to dclk.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IO_OUTPUT_SEL_A {
    #[doc = "0: Normal output"]
    NORMAL = 0,
    #[doc = "1: Register output"]
    R_EGISTER = 1,
}
impl From<IO_OUTPUT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IO_OUTPUT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IO_OUTPUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_OUTPUT_SEL_A {
        match self.bits {
            false => IO_OUTPUT_SEL_A::NORMAL,
            true => IO_OUTPUT_SEL_A::R_EGISTER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IO_OUTPUT_SEL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `R_EGISTER`"]
    #[inline(always)]
    pub fn is_r_egister(&self) -> bool {
        *self == IO_OUTPUT_SEL_A::R_EGISTER
    }
}
#[doc = "Field `io_output_sel` writer - When it is set as '1', the d\\[23:0\\], io0, io1, io3 are sync to dclk."]
pub type IO_OUTPUT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCD_IO_POL_SPEC, IO_OUTPUT_SEL_A, O>;
impl<'a, const O: u8> IO_OUTPUT_SEL_W<'a, O> {
    #[doc = "Normal output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IO_OUTPUT_SEL_A::NORMAL)
    }
    #[doc = "Register output"]
    #[inline(always)]
    pub fn r_egister(self) -> &'a mut W {
        self.variant(IO_OUTPUT_SEL_A::R_EGISTER)
    }
}
impl R {
    #[doc = "Bits 0:23 - LCD output port D\\[23:0\\] polarity control, with independent bit control."]
    #[inline(always)]
    pub fn data_inv(&self) -> DATA_INV_R {
        DATA_INV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    pub unsafe fn io_inv(&self, n: u8) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    pub fn io0_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    pub fn io1_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    pub fn io2_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    pub fn io3_inv(&self) -> IO_INV_R {
        IO_INV_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Set the phase offset of clock and data in hv mode."]
    #[inline(always)]
    pub fn dclk_sel(&self) -> DCLK_SEL_R {
        DCLK_SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - When it is set as '1', the d\\[23:0\\], io0, io1, io3 are sync to dclk."]
    #[inline(always)]
    pub fn io_output_sel(&self) -> IO_OUTPUT_SEL_R {
        IO_OUTPUT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - LCD output port D\\[23:0\\] polarity control, with independent bit control."]
    #[inline(always)]
    #[must_use]
    pub fn data_inv(&mut self) -> DATA_INV_W<0> {
        DATA_INV_W::new(self)
    }
    #[doc = "Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn io_inv<const O: u8>(&mut self) -> IO_INV_W<O> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 24 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io0_inv(&mut self) -> IO_INV_W<24> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 25 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io1_inv(&mut self) -> IO_INV_W<25> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 26 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io2_inv(&mut self) -> IO_INV_W<26> {
        IO_INV_W::new(self)
    }
    #[doc = "Bit 27 - Enable invert function of IO\\[i\\]"]
    #[inline(always)]
    #[must_use]
    pub fn io3_inv(&mut self) -> IO_INV_W<27> {
        IO_INV_W::new(self)
    }
    #[doc = "Bits 28:30 - Set the phase offset of clock and data in hv mode."]
    #[inline(always)]
    #[must_use]
    pub fn dclk_sel(&mut self) -> DCLK_SEL_W<28> {
        DCLK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - When it is set as '1', the d\\[23:0\\], io0, io1, io3 are sync to dclk."]
    #[inline(always)]
    #[must_use]
    pub fn io_output_sel(&mut self) -> IO_OUTPUT_SEL_W<31> {
        IO_OUTPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD IO Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_io_pol](index.html) module"]
pub struct LCD_IO_POL_SPEC;
impl crate::RegisterSpec for LCD_IO_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_io_pol::R](R) reader structure"]
impl crate::Readable for LCD_IO_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_io_pol::W](W) writer structure"]
impl crate::Writable for LCD_IO_POL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lcd_io_pol to value 0"]
impl crate::Resettable for LCD_IO_POL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
